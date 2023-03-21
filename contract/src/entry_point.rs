//! Entry points of the contract.
//!
//! Some methods have entry points that are not listed in the returned `EntryPoints` object.
//! These are either optional or only contextually available. See the documentation of the
//! individual methods for more information.

use casper_types::{ContractPackageHash, Key, U256};

use crate::entrypoints;

entrypoints! {
    /// Overloaded method. Arguments depend on concrete marketplace mode.
    ///
    /// Common arguments
    /// `post_id`: [`u64`] - id of the order to be bid on\
    /// `amount`: [`u64`] - amount of tokens to bid\
    [public session] fn bid(post_id: u64, amount: U256) -> () = crate::bid;

    /// Remove an existing bid from an order.
    [public session] fn unbid() -> () = crate::unbid;

    /// Overloaded method. Arguments depend on concrete marketplace mode.
    ///
    /// Common arguments
    /// - `nft_contract`: [`casper_types::ContractPackageHash`] - NFT package hash\
    ///
    /// Either of the following arguments, but not both, depending on NFT mode:\
    /// - `token_id`: [`u64`] - NFT identifier (if related NFT uses ordinals)\
    /// - `token_hash`: [`alloc::string::String`] - NFT hash (if related NFT uses hashes)\
    ///
    /// Order-book arguments:\
    /// - `quote_token_contract`: [`casper_types::ContractPackageHash`] - token used as quote currency\
    /// - `price`: [`u64`] - price of the NFT in quote currency\
    ///
    /// Auction arguments:\
    /// TODO\
    ///
    /// Return value:\
    /// [`u64`] posted id of the order\
    [public session] fn post(
        nft_contract: ContractPackageHash
    ) -> u64 = crate::post;

    /// Cancel order.
    ///
    /// Arguments:
    /// - `post_id`: [`u64`] - id of the order to be cancelled\
    [public session] fn cancel(
        post_id: u64
    ) -> () = crate::cancel;

    /// Get real owner of an NFT if it is currently on sale.
    ///
    /// Either of the following arguments, but not both, depending on NFT mode:\
    /// - `token_id`: [`u64`] - NFT identifier (if related NFT uses ordinals)\
    /// - `token_hash`: [`alloc::string::String`] - NFT hash (if related NFT uses hashes)\
    [public contract] fn get_real_owner(
        nft_contract: ContractPackageHash,
    ) -> Key = crate::get_real_owner;
}
