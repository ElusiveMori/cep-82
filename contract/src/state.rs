//! Accessors for contract data
//!
//! The contract uses the following named key scheme:  
//! Common:  
//!
//! Order-book:  
//!
//! Auction:  
//! TODO

use alloc::{string::String, vec::Vec};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ContractPackageHash, Key, U256};

use crate::{named_keys, serializable_structs, MarketError, TokenIdentifier};

serializable_structs! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NftContractMetadata {
        pub package: ContractPackageHash,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TokenContractMetadata {
        pub package: ContractPackageHash,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OrderbookEntry {
        pub quote_contract_id: u64,
        pub nft_contract_id: u64,

        pub owner: Key,
        pub token_id: TokenIdentifier,
        pub price: U256,
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct Counters {
        pub nft_contract_id: u64,
        pub token_contract_id: u64,
        pub post_id: u64,
    }
}

named_keys! {
    all_named_keys:
    // Common named keys
    dict nft_contract_metadata_by_id: NftContractMetadata;
    dict nft_contract_id_by_package_hash: u64;
    dict token_contract_metadata_by_id: TokenContractMetadata;
    dict token_contract_id_by_package_hash: u64;
    val counters: Counters = Counters::default();

    // Order book specificic named keys
    dict orderbook_entry_by_id: OrderbookEntry;
}

// // Common variables
// named_key!(dict nft_contract_metadata_by_id: NftContractMetadata);
// named_key!(dict nft_contract_id_by_package_hash: u64);
// named_key!(dict token_contract_metadata_by_id: TokenContractMetadata);
// named_key!(dict token_contract_id_by_package_hash: u64);
// named_key!(val counters: Counters = Counters::default());

// // Order book specificic variables
// named_key!(dict orderbook_entry_by_id: OrderbookEntry);

fn package_hash_key(package: ContractPackageHash) -> String {
    BASE64_STANDARD_NO_PAD.encode(package.as_bytes())
}

fn u64_key(id: u64) -> String {
    BASE64_STANDARD_NO_PAD.encode(id.to_be_bytes().as_ref())
}

impl NftContractMetadata {
    pub fn by_id(id: u64) -> Self {
        nft_contract_metadata_by_id::try_read(&u64_key(id))
            .unwrap_or_revert_with(MarketError::UnsupportedNFTContract)
    }

    pub fn by_package_hash(package: ContractPackageHash) -> (u64, Self) {
        nft_contract_id_by_package_hash::try_read(&package_hash_key(package))
            .and_then(|id| {
                nft_contract_metadata_by_id::try_read(&u64_key(id)).map(|metadata| (id, metadata))
            })
            .unwrap_or_revert_with(MarketError::UnsupportedNFTContract)
    }

    pub fn write(self, id: u64) {
        nft_contract_id_by_package_hash::write(&package_hash_key(self.package), id);
        nft_contract_metadata_by_id::write(&u64_key(id), self);
    }
}

impl TokenContractMetadata {
    pub fn by_id(id: u64) -> Self {
        token_contract_metadata_by_id::try_read(&u64_key(id))
            .unwrap_or_revert_with(MarketError::UnsupportedFungibleTokenContract)
    }

    pub fn by_package_hash(package: ContractPackageHash) -> (u64, Self) {
        token_contract_id_by_package_hash::try_read(&package_hash_key(package))
            .and_then(|id| {
                token_contract_metadata_by_id::try_read(&u64_key(id)).map(|metadata| (id, metadata))
            })
            .unwrap_or_revert_with(MarketError::UnsupportedFungibleTokenContract)
    }

    pub fn write(self, id: u64) {
        token_contract_id_by_package_hash::write(&package_hash_key(self.package), id);
        token_contract_metadata_by_id::write(&u64_key(id), self);
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
        orderbook_entry_by_id::try_read(&u64_key(id))
            .unwrap_or_revert_with(MarketError::UnknownPostId)
    }

    pub fn write(self, id: u64) {
        orderbook_entry_by_id::write(&u64_key(id), self);
    }

    pub fn remove(id: u64) {
        orderbook_entry_by_id::remove(&u64_key(id));
    }
}
