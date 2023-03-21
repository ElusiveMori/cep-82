#![no_std]

extern crate alloc;

pub mod entry_point;
pub mod ext;
pub mod state;
pub mod util;

use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::{
    runtime::{self, revert},
    storage,
};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    ApiError, CLValue, ContractPackageHash, Key, NamedArg, U256,
};
use util::{call_stack::CallStackElementEx, contract_api::try_get_named_arg};

use crate::{
    state::{Counters, OrderbookEntry, TokenContractMetadata},
    util::call_stack,
};

pub const NK_ACCESS_UREF: &str = "cep_82_contract_uref";
pub const NK_CONTRACT: &str = "cep_82_contract";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenIdentifier {
    Index(u64),
    Hash(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MarketError {
    InvalidTokenIdentifier,
    InvalidMethodAccess,
    InvalidPaymentAmount,

    UnsupportedFungibleTokenContract,
    UnsupportedNFTContract,

    UnknownPostId,
}

impl From<MarketError> for ApiError {
    fn from(error: MarketError) -> Self {
        ApiError::User(error as u16)
    }
}

impl TokenIdentifier {
    pub fn load_from_runtime_args() -> Self {
        if let Ok(token_id) = try_get_named_arg::<u64>("token_id") {
            TokenIdentifier::Index(token_id)
        } else if let Ok(token_hash) = try_get_named_arg::<String>("token_hash") {
            TokenIdentifier::Hash(token_hash)
        } else {
            revert(MarketError::InvalidTokenIdentifier);
        }
    }

    pub fn to_named_arg(&self) -> NamedArg {
        match self {
            TokenIdentifier::Index(index) => {
                NamedArg::new("token_id".into(), CLValue::from_t(*index).unwrap())
            }
            TokenIdentifier::Hash(hash) => {
                NamedArg::new("token_hash".into(), CLValue::from_t(hash.clone()).unwrap())
            }
        }
    }
}

impl ToBytes for TokenIdentifier {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        match self {
            TokenIdentifier::Index(index) => {
                let mut result = bytesrepr::allocate_buffer(self)?;
                result.push(0);
                result.append(&mut index.to_bytes()?);
                Ok(result)
            }
            TokenIdentifier::Hash(hash) => {
                let mut result = bytesrepr::allocate_buffer(self)?;
                result.push(1);
                result.append(&mut hash.to_bytes()?);
                Ok(result)
            }
        }
    }

    fn serialized_length(&self) -> usize {
        match self {
            TokenIdentifier::Index(index) => 1 + index.serialized_length(),
            TokenIdentifier::Hash(hash) => 1 + hash.serialized_length(),
        }
    }
}

impl FromBytes for TokenIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (tag, rem) = u8::from_bytes(bytes)?;
        match tag {
            0 => {
                let (index, rem) = u64::from_bytes(rem)?;
                Ok((TokenIdentifier::Index(index), rem))
            }
            1 => {
                let (hash, rem) = String::from_bytes(rem)?;
                Ok((TokenIdentifier::Hash(hash), rem))
            }
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

pub fn install() {
    let named_keys = state::all_named_keys().into_iter().collect::<_>();
    let entry_points = entry_point::all_entrypoints().into();

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    runtime::put_key(NK_ACCESS_UREF, access_uref.into());
    runtime::put_key(NK_CONTRACT, contract_hash.into());
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
    let nft_contract = TokenContractMetadata::by_id(entry.nft_contract_id);

    ext::erc20::transfer_from(quote_contract.package, bidder, owner, entry.price);
    ext::cep78::transfer(nft_contract.package, entry.token_id, this, bidder);

    OrderbookEntry::remove(post_id);
}

pub fn unbid() {
    // TODO: currently no implementation, as unbid only makes sense in auction
    // scenarios, which are currently not implemented
}

pub fn post(nft_contract: ContractPackageHash) -> u64 {
    let source_key = call_stack::caller().key();
    let target_key: Key = call_stack::current_package().into();

    read_arg!(quote_token_contract: ContractPackageHash);
    read_arg!(price: U256);
    read_token_id!(token_id);

    ext::cep78::transfer(nft_contract, token_id.clone(), source_key, target_key);

    let mut counters = Counters::read();
    let post_id = counters.post_id;
    counters.post_id += 1;
    counters.write();

    let (quote_contract_id, _) = TokenContractMetadata::by_package_hash(quote_token_contract);
    let (nft_contract_id, _) = TokenContractMetadata::by_package_hash(nft_contract);

    let entry = OrderbookEntry {
        owner: source_key,
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

    let nft_contract = TokenContractMetadata::by_id(entry.nft_contract_id);
    ext::cep78::transfer(nft_contract.package, entry.token_id, this, caller);

    OrderbookEntry::remove(post_id);
}

pub fn get_real_owner(nft_contract: ContractPackageHash) {
    todo!()
}
