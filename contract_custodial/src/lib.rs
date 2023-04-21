#![no_std]

use casper_contract::contract_api::runtime::call_versioned_contract;
use contract_common::{
    call_stack::{self, CallStackElementEx},
    debug_log, ext,
    prelude::*,
    qlog, store_named_key_incremented,
    token::TokenIdentifier,
};

use crate::state::check_is_token_known;

extern crate alloc;

pub mod entry_point;
pub mod state;

pub const NK_ACCESS_UREF: &str = "cep82_custodial_uref";
pub const NK_CONTRACT: &str = "cep82_custodial";

pub const NAME: &str = "custodial";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CustodialError {
    AlreadyClaimed,
    AlreadyDelegated,
    UndelegationNotAllowed,

    NoAccess,
    NoDelegate,
    UnknownToken,

    PaymentTokenNotWhitelisted,
    MarketplaceNotWhitelisted,

    Overflow,
}

impl From<CustodialError> for ApiError {
    fn from(error: CustodialError) -> Self {
        ApiError::User(error as u16)
    }
}

fn install(
    wrapped_contract: ContractPackageHash,
    whitelisted_payment_tokens: Vec<ContractPackageHash>,
    whitelisted_marketplaces: Vec<ContractPackageHash>,
    royalty_percent: U256,
    manager: Key,
) {
    let named_keys = state::init_all(manager, wrapped_contract, royalty_percent)
        .into_iter()
        .collect::<_>();

    let entry_points = entry_point::all_entrypoints().into();

    init(whitelisted_marketplaces, whitelisted_payment_tokens);

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    store_named_key_incremented(access_uref.into(), NK_ACCESS_UREF);
    store_named_key_incremented(contract_hash.into(), NK_CONTRACT);
}

fn init(
    whitelisted_marketplaces: Vec<ContractPackageHash>,
    whitelisted_payment_tokens: Vec<ContractPackageHash>,
) {
    for token in whitelisted_payment_tokens {
        state::whitelisted_payment_tokens::write(&b64_cl(&token), true);
    }

    for marketplace in whitelisted_marketplaces {
        state::whitelisted_marketplaces::write(&b64_cl(&marketplace), true);
    }
}

fn transfer(
    token_id: TokenIdentifier,
    _source_key: Key,
    target_key: Key,
    payment_source_key: Key,
    payment_token: ContractPackageHash,
    payment_amount: U256,
) {
    check_is_token_known(&token_id);

    // May only be called by a contract
    let caller = call_stack::caller()
        .contract_package()
        .unwrap_or_revert_with(CustodialError::NoAccess);

    // Delegate must be set
    let delegate = state::delegate_of(&token_id).unwrap_or_revert_with(CustodialError::NoDelegate);

    // Only the delegate may transfer the token
    ensure_eq!(delegate, caller, CustodialError::NoAccess);

    // Ensure that the payment token is whitelisted
    ensure!(
        state::is_payment_token_whitelisted(payment_token),
        CustodialError::PaymentTokenNotWhitelisted
    );

    let manager = state::manager::read();

    let royalty_amount = calculate_royalty_inner(&token_id, payment_token, payment_amount);
    qlog!("royalty_amount: {}", royalty_amount);

    // Transfer royalty to manager
    ext::erc20::transfer_from(payment_token, payment_source_key, manager, royalty_amount);

    state::set_real_owner(&token_id, target_key);
    state::set_delegate(&token_id, None);
}

fn balance_of(token_owner: Key) -> U256 {
    if let Some(owner_data) = state::owner_data::try_read(&b64_cl(&token_owner)) {
        owner_data.owned_count.into()
    } else {
        0u64.into()
    }
}

fn owner_of(token_id: TokenIdentifier) -> Key {
    state::real_owner_of(&token_id)
}

fn metadata(token_id: TokenIdentifier) -> String {
    ext::cep78::metadata(state::wrapped_contract::read(), token_id)
}

fn set_delegate(token_id: TokenIdentifier, new_delegate: Option<ContractPackageHash>) {
    if let Some(new_delegate) = new_delegate {
        let real_owner = state::real_owner_of(&token_id);
        let caller = call_stack::caller().key();

        // Token must be undelegated before it can be delegated again
        ensure!(
            state::delegate_of(&token_id).is_none(),
            CustodialError::AlreadyDelegated
        );

        // May only be called by token owner
        ensure_eq!(caller, real_owner, CustodialError::NoAccess);

        // Delegate must be whitelisted
        ensure!(
            state::is_marketplace_whitelisted(new_delegate),
            CustodialError::MarketplaceNotWhitelisted
        );

        state::set_delegate(&token_id, Some(new_delegate));
    } else {
        let real_owner = state::real_owner_of(&token_id);
        let caller = call_stack::caller().key();

        // May only be called by token owner
        ensure_eq!(caller, real_owner, CustodialError::NoAccess);

        // The current delegate must approve the undelegation
        let delegate =
            state::delegate_of(&token_id).unwrap_or_revert_with(CustodialError::NoDelegate);
        let can_undelegate = call_versioned_contract::<bool>(
            delegate,
            None,
            "request_undelegate",
            vec![token_id.to_named_arg(), named_arg!(real_owner)].into(),
        );

        ensure!(can_undelegate, CustodialError::UndelegationNotAllowed);

        state::set_delegate(&token_id, None);
    }
}

fn delegate(token_id: TokenIdentifier) -> Option<ContractPackageHash> {
    state::delegate_of(&token_id)
}

fn claim(token_id: TokenIdentifier, owner: Key) {
    let caller = call_stack::caller().key();

    // May only be called by manager
    ensure_eq!(caller, state::manager::read(), CustodialError::NoAccess);

    // Token must be unmanaged
    ensure!(
        state::try_real_owner_of(&token_id).is_none(),
        CustodialError::AlreadyClaimed
    );

    state::set_real_owner(&token_id, owner);
}

fn calculate_royalty_inner(
    _token_id: &TokenIdentifier,
    _payment_token: ContractPackageHash,
    payment_amount: U256,
) -> U256 {
    let royalty_percent = state::royalty_percent::read();
    payment_amount
        .checked_mul(royalty_percent)
        .unwrap_or_revert_with(CustodialError::Overflow)
        .checked_div(10000u64.into())
        .unwrap_or_revert_with(CustodialError::Overflow)
}

fn calculate_royalty(
    token_id: TokenIdentifier,
    payment_token: ContractPackageHash,
    payment_amount: U256,
) -> U256 {
    calculate_royalty_inner(&token_id, payment_token, payment_amount)
}
