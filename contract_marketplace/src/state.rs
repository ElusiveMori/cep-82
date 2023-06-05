use alloc::{format, string::String, vec::Vec};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use casper_contract::contract_api::runtime;
use casper_types::{ContractPackageHash, Key, URef, U512};
use contract_common::{b64_cl, o_unwrap, token::TokenIdentifier};

use crate::{named_keys, serializable_structs, MarketError};

serializable_structs! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NftContractMetadata {
        pub nft_package: ContractPackageHash,
        pub custodial_package: Option<ContractPackageHash>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OrderbookEntry {
        pub nft_contract_id: u64,

        pub owner: Key,
        pub token_id: TokenIdentifier,
        pub price: U512,
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct Counters {
        pub nft_contract_id: u64,
        pub token_contract_id: u64,
        pub post_id: u64,
    }
}

named_keys! {
    all_named_keys():
    // Common named keys
    dict nft_contract_metadata_by_id: NftContractMetadata;
    dict nft_contract_id_by_package_hash: u64;
    val counters: Counters = Counters::default();

    // Order book specificic named keys
    dict orderbook_entry_by_id: OrderbookEntry;
    dict post_id_by_token_id: u64;
}

fn package_hash_key(package: ContractPackageHash) -> String {
    BASE64_STANDARD_NO_PAD.encode(package.as_bytes())
}

fn u64_key(id: u64) -> String {
    BASE64_STANDARD_NO_PAD.encode(id.to_be_bytes().as_ref())
}

impl NftContractMetadata {
    pub fn by_id(id: u64) -> Self {
        o_unwrap!(
            nft_contract_metadata_by_id::try_read(&u64_key(id)),
            MarketError::UnsupportedNFTContract
        )
    }

    pub fn by_package_hash(package: ContractPackageHash) -> (u64, Self) {
        o_unwrap!(
            nft_contract_id_by_package_hash::try_read(&package_hash_key(package)).and_then(|id| {
                nft_contract_metadata_by_id::try_read(&u64_key(id)).map(|metadata| (id, metadata))
            }),
            MarketError::UnsupportedNFTContract
        )
    }

    pub fn write(self, id: u64) {
        nft_contract_id_by_package_hash::write(&package_hash_key(self.nft_package), id);
        nft_contract_metadata_by_id::write(&u64_key(id), self);
    }
}

impl Counters {
    pub fn read() -> Self {
        counters::read()
    }

    pub fn write(self) {
        counters::write(self);
    }
}

impl OrderbookEntry {
    pub fn by_id(id: u64) -> Self {
        o_unwrap!(
            orderbook_entry_by_id::try_read(&u64_key(id)),
            MarketError::UnknownPostId
        )
    }

    pub fn write(self, id: u64) {
        orderbook_entry_by_id::write(&u64_key(id), self);
    }

    pub fn remove(id: u64) {
        orderbook_entry_by_id::remove(&u64_key(id));
    }
}

pub fn post_id_by_token_id(token_id: &TokenIdentifier) -> Option<u64> {
    post_id_by_token_id::try_read(&b64_cl(token_id))
}

pub fn set_post_id_by_token_id(token_id: &TokenIdentifier, id: Option<u64>) {
    match id {
        Some(id) => post_id_by_token_id::write(&b64_cl(token_id), id),
        None => post_id_by_token_id::remove(&b64_cl(token_id)),
    }
}

pub fn set_target_purse_by_post_id(post_id: u64, purse: URef) {
    runtime::put_key(&format!("target_purse_{post_id}"), purse.into());
}

pub fn unset_target_purse_by_post_id(post_id: u64) {
    runtime::remove_key(&format!("target_purse_{post_id}"));
}

pub fn target_purse_by_post_id(post_id: u64) -> Option<URef> {
    runtime::get_key(&format!("target_purse_{post_id}")).and_then(|k| k.into_uref())
}
