//! Entry points of the contract.
//!
//! Some methods have entry points that are not listed in the returned `EntryPoints` object.
//! These are either optional or only contextually available. See the documentation of the
//! individual methods for more information.

use contract_common::{prelude::*, token::TokenIdentifier};

entrypoint! {
    [install] fn call(
        wrapped_contract: ContractPackageHash,
        whitelisted_payment_tokens: Vec<ContractPackageHash>,
        whitelisted_marketplaces: Vec<ContractPackageHash>,
        royalty_percent: U256,
        manager: Key,
    ) -> () = crate::install
}

entrypoints! {
    [public contract] fn transfer(
        token_id: TokenIdentifier,
        source_key: Key,
        target_key: Key,
        payment_source_key: Key,
        payment_token: ContractPackageHash,
        payment_amount: U256,
    ) -> () = crate::transfer;

    [public contract] fn balance_of(token_owner: Key) -> U256 = crate::balance_of;
    [public contract] fn owner_of(token_id: TokenIdentifier) -> Key = crate::owner_of;
    [public contract] fn metadata(token_id: TokenIdentifier) -> String = crate::metadata;
    [public contract] fn delegate(token_id: TokenIdentifier) -> Option<ContractPackageHash> = crate::delegate;
    [public contract] fn calculate_royalty(
        token_id: TokenIdentifier,
        payment_token: ContractPackageHash,
        payment_amount: U256,
    ) -> U256 = crate::calculate_royalty;

    [public contract] fn set_delegate(token_id: TokenIdentifier, new_delegate: Option<ContractPackageHash>) -> () = crate::set_delegate;
    [public contract] fn claim(token_id: TokenIdentifier, owner: Key) -> () = crate::claim;
}
