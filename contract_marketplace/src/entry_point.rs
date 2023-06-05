//! Entry points of the contract.
//!
//! Some methods have entry points that are not listed in the returned `EntryPoints` object.
//! These are either optional or only contextually available. See the documentation of the
//! individual methods for more information.

use casper_types::{ContractPackageHash, URef, U512};
use contract_common::{entrypoint, entrypoints, token::TokenIdentifier};

entrypoint! {
    [install] fn call() -> () = crate::install
}

entrypoints! {
    [public contract] fn bid(post_id: u64, source_purse: URef, amount: U512) -> () = crate::bid;

    [public contract] fn post(
        nft_contract: ContractPackageHash,
        token_id: TokenIdentifier,
        target_purse: URef,
        price: U512,
    ) -> u64 = crate::post;

    [public contract] fn cancel(
        post_id: u64
    ) -> () = crate::cancel;

    [public contract] fn register_cep78_contract(
        nft_package: ContractPackageHash,
        custodial_package: Option<ContractPackageHash>,
    ) -> () = crate::register_cep78_contract;
}
