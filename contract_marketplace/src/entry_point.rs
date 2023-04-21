//! Entry points of the contract.
//!
//! Some methods have entry points that are not listed in the returned `EntryPoints` object.
//! These are either optional or only contextually available. See the documentation of the
//! individual methods for more information.

use casper_types::{ContractPackageHash, Key, U256};
use contract_common::{entrypoint, entrypoints, token::TokenIdentifier};

entrypoint! {
    [install] fn call() -> () = crate::install
}

entrypoints! {
    [public contract] fn bid(post_id: u64, amount: U256) -> () = crate::bid;

    [public contract] fn post(
        token_id: TokenIdentifier,
        quote_token_contract: ContractPackageHash,
        price: U256,
        nft_contract: ContractPackageHash
    ) -> u64 = crate::post;

    [public contract] fn cancel(
        post_id: u64
    ) -> () = crate::cancel;

    [public contract] fn register_erc20_contract(
        package: ContractPackageHash,
    ) -> () = crate::register_erc20_contract;

    [public contract] fn register_cep78_contract(
        package: ContractPackageHash,
        is_cep82_compliant: bool,
    ) -> () = crate::register_cep78_contract;

    [public contract] fn request_undelegate(
        token_id: TokenIdentifier,
        real_owner: Key,
    ) -> bool = crate::request_undelegate;
}
