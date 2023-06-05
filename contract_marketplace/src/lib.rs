#![no_std]

extern crate alloc;

pub mod entry_point;
pub mod state;

use contract_common::{
    call_stack::{self, CallStackElementEx},
    ext, o_unwrap,
    prelude::*,
    r_unwrap, store_named_key_incremented,
    token::TokenIdentifier,
};
use state::{unset_target_purse_by_post_id, NftContractMetadata, OrderbookEntry};

use crate::state::{set_target_purse_by_post_id, target_purse_by_post_id, Counters};

pub const NK_ACCESS_UREF: &str = "cep82_marketplace_uref";
pub const NK_CONTRACT: &str = "cep82_marketplace";

pub const NAME: &str = "marketplace";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MarketError {
    InvalidMethodAccess,
    InvalidPaymentAmount,

    MustBeApproved,

    UnsupportedNFTContract,

    UnknownPostId,
    UnknownTokenId,

    ArithmeticOverflow,
}

impl From<MarketError> for ApiError {
    fn from(error: MarketError) -> Self {
        ApiError::User(error as u16)
    }
}

pub fn install() {
    let named_keys = state::all_named_keys().into_iter().collect::<_>();
    let entry_points = entry_point::all_entrypoints().into();

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    store_named_key_incremented(access_uref.into(), NK_ACCESS_UREF);
    store_named_key_incremented(contract_hash.into(), NK_CONTRACT);
}

pub fn bid(post_id: u64, source_purse: URef, amount: U512) {
    let entry = OrderbookEntry::by_id(post_id);
    let bidder = call_stack::caller().key();
    let owner = entry.owner;

    if amount < entry.price {
        revert(MarketError::InvalidPaymentAmount);
    }

    let nft_contract = NftContractMetadata::by_id(entry.nft_contract_id);

    if let Some(custodial_package) = nft_contract.custodial_package {
        let royalty_amount = ext::cep82::custodial::calculate_royalty(
            custodial_package,
            nft_contract.nft_package,
            &entry.token_id,
            entry.price,
        );

        let royalty_purse = casper_contract::contract_api::system::create_purse();
        let remaining_amount = amount
            .checked_sub(royalty_amount)
            .unwrap_or_revert_with(MarketError::ArithmeticOverflow);

        r_unwrap!(
            casper_contract::contract_api::system::transfer_from_purse_to_purse(
                source_purse,
                royalty_purse,
                royalty_amount,
                None,
            )
        );

        let target_purse = o_unwrap!(target_purse_by_post_id(post_id), MarketError::UnknownPostId);
        r_unwrap!(
            casper_contract::contract_api::system::transfer_from_purse_to_purse(
                source_purse,
                target_purse,
                remaining_amount,
                None,
            )
        );

        ext::cep82::custodial::pay_royalty(
            custodial_package,
            nft_contract.nft_package,
            &entry.token_id,
            royalty_purse,
            bidder,
            owner,
            bidder,
            entry.price,
        );
    }

    ext::cep78::transfer(
        nft_contract.nft_package,
        &entry.token_id,
        entry.owner,
        bidder,
    );

    unset_target_purse_by_post_id(post_id);
    OrderbookEntry::remove(post_id);
}

pub fn post(
    nft_contract: ContractPackageHash,
    token_id: TokenIdentifier,
    target_purse: URef,
    price: U512,
) -> u64 {
    let caller = call_stack::caller().key();

    let (nft_contract_id, _) = NftContractMetadata::by_package_hash(nft_contract);

    let approved = o_unwrap!(
        ext::cep78::get_approved(nft_contract, &token_id),
        MarketError::MustBeApproved
    );

    let this: Key = call_stack::current_contract().into();
    ensure_eq!(approved, this, MarketError::MustBeApproved);

    let owner = ext::cep78::owner_of(nft_contract, &token_id);
    ensure_eq!(owner, caller, MarketError::InvalidMethodAccess);

    let mut counters = Counters::read();
    let post_id = counters.post_id;
    counters.post_id += 1;
    counters.write();
    state::set_post_id_by_token_id(&token_id, Some(post_id));

    let entry = OrderbookEntry {
        owner,
        nft_contract_id,
        token_id,
        price,
    };

    set_target_purse_by_post_id(post_id, target_purse);

    entry.write(post_id);

    post_id
}

pub fn cancel(post_id: u64) {
    let caller = call_stack::caller().key();
    let entry = OrderbookEntry::by_id(post_id);

    if entry.owner != caller {
        revert(MarketError::InvalidMethodAccess);
    }

    state::set_post_id_by_token_id(&entry.token_id, None);
    OrderbookEntry::remove(post_id);
}

pub fn register_cep78_contract(
    nft_package: ContractPackageHash,
    custodial_package: Option<ContractPackageHash>,
) {
    let mut counters = Counters::read();
    let contract_id = counters.post_id;
    counters.post_id += 1;
    counters.write();

    let entry = NftContractMetadata {
        nft_package,
        custodial_package,
    };
    entry.write(contract_id);
}
