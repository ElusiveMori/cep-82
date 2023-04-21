#![no_std]

extern crate alloc;

pub mod entry_point;
pub mod state;

use contract_common::{
    call_stack::{self, CallStackElementEx},
    ext,
    prelude::*,
    qlog, store_named_key_incremented,
    token::TokenIdentifier,
};
use state::{NftContractMetadata, OrderbookEntry, TokenContractMetadata};

use crate::state::Counters;

pub const NK_ACCESS_UREF: &str = "cep82_marketplace_uref";
pub const NK_CONTRACT: &str = "cep82_marketplace";

pub const NAME: &str = "marketplace";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MarketError {
    InvalidTokenIdentifier,
    InvalidMethodAccess,
    InvalidPaymentAmount,

    MustBeDelegated,

    UnsupportedFungibleTokenContract,
    UnsupportedNFTContract,

    UnknownPostId,
    UnknownTokenId,

    ArithmeticOverflow,
    RoyaltyMismatch,
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

pub fn bid(post_id: u64, amount: U256) {
    let entry = OrderbookEntry::by_id(post_id);
    let bidder = call_stack::caller().key();
    let owner = entry.owner;
    let this = call_stack::current_package().into();

    if amount < entry.price {
        revert(MarketError::InvalidPaymentAmount);
    }

    let quote_contract = TokenContractMetadata::by_id(entry.quote_contract_id);
    let nft_contract = NftContractMetadata::by_id(entry.nft_contract_id);

    if nft_contract.is_cep82_compliant {
        ext::erc20::transfer_from(quote_contract.package, bidder, this, entry.price);
        let expected_royalty = ext::cep82::custodial::calculate_royalty(
            nft_contract.package,
            &entry.token_id,
            quote_contract.package,
            entry.price,
        );
        qlog!("expected_royalty: {}", expected_royalty);

        ext::erc20::approve(
            quote_contract.package,
            nft_contract.package.into(),
            expected_royalty,
        );

        let before_royalty = ext::erc20::balance_of(quote_contract.package, this);
        ext::cep82::custodial::transfer(
            nft_contract.package,
            &entry.token_id,
            owner,
            bidder,
            this,
            quote_contract.package,
            amount,
        );
        let after_royalty = ext::erc20::balance_of(quote_contract.package, this);

        let royalty = before_royalty
            .checked_sub(after_royalty)
            .unwrap_or_revert_with(MarketError::ArithmeticOverflow);

        ensure_eq!(royalty, expected_royalty, MarketError::RoyaltyMismatch);

        let remaining = amount
            .checked_sub(royalty)
            .unwrap_or_revert_with(MarketError::ArithmeticOverflow);
        ext::erc20::transfer(quote_contract.package, owner, remaining);
    } else {
        ext::erc20::transfer_from(quote_contract.package, bidder, owner, entry.price);
        ext::cep78::transfer(nft_contract.package, &entry.token_id, this, bidder);
    }

    OrderbookEntry::remove(post_id);
}

pub fn post(
    token_id: TokenIdentifier,
    quote_token_contract: ContractPackageHash,
    price: U256,
    nft_contract: ContractPackageHash,
) -> u64 {
    let caller = call_stack::caller().key();

    let (quote_contract_id, _) = TokenContractMetadata::by_package_hash(quote_token_contract);
    let (nft_contract_id, nft_metadata) = NftContractMetadata::by_package_hash(nft_contract);

    if nft_metadata.is_cep82_compliant {
        let delegate = ext::cep82::custodial::delegate(nft_contract, &token_id);

        ensure_eq!(
            delegate,
            Some(call_stack::current_package()),
            MarketError::MustBeDelegated
        );
    } else {
        let target_key: Key = call_stack::current_package().into();
        ext::cep78::transfer(nft_contract, &token_id, caller, target_key);
    }

    let owner = ext::cep78::owner_of(nft_contract, &token_id);
    ensure_eq!(owner, caller, MarketError::InvalidMethodAccess);

    let mut counters = Counters::read();
    let post_id = counters.post_id;
    counters.post_id += 1;
    counters.write();
    state::set_post_id_by_token_id(&token_id, Some(post_id));

    let entry = OrderbookEntry {
        owner,
        quote_contract_id,
        nft_contract_id,
        token_id,
        price,
    };

    entry.write(post_id);

    post_id
}

pub fn cancel(post_id: u64) {
    let caller = call_stack::caller().key();
    let this = call_stack::current_package().into();
    let entry = OrderbookEntry::by_id(post_id);

    if entry.owner != caller {
        revert(MarketError::InvalidMethodAccess);
    }

    let nft_contract = NftContractMetadata::by_id(entry.nft_contract_id);

    if !nft_contract.is_cep82_compliant {
        ext::cep78::transfer(nft_contract.package, &entry.token_id, this, caller);
    }

    state::set_post_id_by_token_id(&entry.token_id, None);
    OrderbookEntry::remove(post_id);
}

pub fn register_erc20_contract(package: ContractPackageHash) {
    let mut counters = Counters::read();
    let contract_id = counters.post_id;
    counters.post_id += 1;
    counters.write();

    let entry = TokenContractMetadata { package };
    entry.write(contract_id);
}

pub fn register_cep78_contract(package: ContractPackageHash, is_cep82_compliant: bool) {
    let mut counters = Counters::read();
    let contract_id = counters.post_id;
    counters.post_id += 1;
    counters.write();

    let entry = NftContractMetadata {
        package,
        is_cep82_compliant,
    };
    entry.write(contract_id);
}

pub fn request_undelegate(token_id: TokenIdentifier, _real_owner: Key) -> bool {
    state::post_id_by_token_id(&token_id).is_none()
}
