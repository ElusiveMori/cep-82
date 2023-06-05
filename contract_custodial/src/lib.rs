#![no_std]

use alloc::{collections::BTreeMap, string::ToString};
use contract_common::{
    call_stack::CallStackElementEx, o_unwrap, prelude::*, store_named_key_incremented,
    token::TokenIdentifier,
};
use state::{RoyaltyPaymentState, RoyaltyStructure};

extern crate alloc;

mod bytes;
pub mod entry_point;
pub mod state;

pub const NK_ACCESS_UREF: &str = "cep82_custodial_uref";
pub const NK_CONTRACT: &str = "cep82_custodial";
pub const NK_ROYALTY_PURSE: &str = "royalty_purse";

pub const NAME: &str = "custodial";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CustodialError {
    MarketplaceNotWhitelisted,
    CallerMustBeContract,
    CallerMustBeApproved,
    SelfTransferForbidden,

    SourceMustBeOwner,
    AlreadyPaid,

    Overflow,
}

impl From<CustodialError> for ApiError {
    fn from(error: CustodialError) -> Self {
        ApiError::User(error as u16)
    }
}

fn install(
    whitelisted_marketplaces: Vec<ContractPackageHash>,
    royalty_structure: RoyaltyStructure,
    manager: Key,
) {
    let royalty_purse = casper_contract::contract_api::system::create_purse();
    let mut named_keys: BTreeMap<_, _> = state::init_all(manager, royalty_structure)
        .into_iter()
        .collect::<_>();

    named_keys.insert(NK_ROYALTY_PURSE.to_string(), royalty_purse.into());

    let entry_points = entry_point::all_entrypoints().into();

    init(whitelisted_marketplaces);

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    store_named_key_incremented(access_uref.into(), NK_ACCESS_UREF);
    store_named_key_incremented(contract_hash.into(), NK_CONTRACT);
}

fn init(whitelisted_marketplaces: Vec<ContractPackageHash>) {
    if whitelisted_marketplaces.is_empty() {
        state::marketplace_whitelist_enabled::write(false);
    } else {
        state::marketplace_whitelist_enabled::write(true);

        for marketplace in whitelisted_marketplaces {
            state::whitelisted_marketplaces::write(&b64_cl(&marketplace), true);
        }
    }
}

fn pay_royalty(
    token_contract: ContractPackageHash,
    token_id: TokenIdentifier,
    source_purse: URef,
    payer: Key,
    source_key: Key,
    target_key: Key,
    payment_amount: U512,
) {
    let royalty_purse = runtime::get_key(NK_ROYALTY_PURSE)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();
    let total_royalty = calculate_royalty_inner(&token_id, payment_amount);

    let caller_contract_hash: Key = o_unwrap!(
        contract_common::call_stack::caller()
            .contract_hash()
            .cloned(),
        CustodialError::CallerMustBeContract
    )
    .into();

    let caller_contract_package = o_unwrap!(
        contract_common::call_stack::caller().contract_package(),
        CustodialError::CallerMustBeContract
    );

    let approved = o_unwrap!(
        contract_common::ext::cep78::get_approved(token_contract, &token_id),
        CustodialError::CallerMustBeApproved
    );

    ensure_eq!(
        caller_contract_hash,
        approved,
        CustodialError::CallerMustBeApproved
    );

    let is_whitelisted = (!state::marketplace_whitelist_enabled::read())
        || state::is_marketplace_whitelisted(caller_contract_package);

    ensure!(is_whitelisted, CustodialError::MarketplaceNotWhitelisted);

    let current_owner = contract_common::ext::cep78::owner_of(token_contract, &token_id);

    ensure_eq!(current_owner, source_key, CustodialError::SourceMustBeOwner);
    ensure_neq!(
        source_key,
        target_key,
        CustodialError::SelfTransferForbidden
    );

    let old_payment_state = state::royalty_payments::try_read(&b64_cl(&token_id));
    if let Some(RoyaltyPaymentState::Paid {
        source_key: paid_source_key,
        ..
    }) = old_payment_state
    {
        ensure_neq!(source_key, paid_source_key, CustodialError::AlreadyPaid)
    }

    contract_api::system::transfer_from_purse_to_purse(
        source_purse,
        royalty_purse,
        total_royalty,
        None,
    )
    .unwrap_or_revert();

    let payment_state = RoyaltyPaymentState::Paid {
        payer,
        source_key,
        amount: total_royalty,
    };

    state::royalty_payments::write(&b64_cl(&token_id), payment_state);
}

fn calculate_royalty_inner(_token_id: &TokenIdentifier, payment_amount: U512) -> U512 {
    let royalty_structure = state::royalty_structure::read();
    royalty_structure.calculate_total_royalty(payment_amount)
}

fn calculate_royalty(
    _token_contract: ContractPackageHash,
    token_id: TokenIdentifier,
    payment_amount: U512,
) -> U512 {
    calculate_royalty_inner(&token_id, payment_amount)
}

fn can_transfer(token_id: TokenIdentifier, source_key: Key, _target_key: Key) -> u8 {
    const PROCEED: u8 = 1;
    const DENY: u8 = 0;

    let key = b64_cl(&token_id);
    let payment_state = state::royalty_payments::read(&key);

    let caller = o_unwrap!(
        contract_common::call_stack::caller().contract_package(),
        CustodialError::CallerMustBeContract
    );

    let current_owner = contract_common::ext::cep78::owner_of(caller, &token_id);

    match payment_state {
        RoyaltyPaymentState::Unpaid => DENY,
        RoyaltyPaymentState::Paid {
            source_key: paid_source_key,
            ..
        } => {
            if source_key == paid_source_key && source_key == current_owner {
                // NB: it is ok to write `Unpaid` here, even though this technically happens *before* the transfer,
                // because an unsuccessful transfer attempt will revert the whole deploy
                state::royalty_payments::write(&key, RoyaltyPaymentState::Unpaid);

                PROCEED
            } else {
                DENY
            }
        }
    }
}
