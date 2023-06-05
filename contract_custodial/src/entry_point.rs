//! Entry points of the contract.
//!
//! Some methods have arguments that are not listed in the returned `EntryPoints` object.
//! These are either optional or only contextually available. See the documentation of the
//! individual methods for more information.

use contract_common::{prelude::*, token::TokenIdentifier};

use crate::state::RoyaltyStructure;

entrypoint! {
    [install] fn call(
        whitelisted_marketplaces: Vec<ContractPackageHash>,
        royalty_structure: RoyaltyStructure,
        manager: Key,
    ) -> () = crate::install
}

entrypoints! {
    [public contract] fn can_transfer(
        token_id: TokenIdentifier,
        source_key: Key,
        target_key: Key
    ) -> u8 = crate::can_transfer;

    [public contract] fn pay_royalty(
        token_contract: ContractPackageHash,
        token_id: TokenIdentifier,
        source_purse: URef,
        payer: Key,
        source_key: Key,
        target_key: Key,
        payment_amount: U512,
    ) -> () = crate::pay_royalty;

    [public contract] fn calculate_royalty(
        token_contract: ContractPackageHash,
        token_id: TokenIdentifier,
        payment_amount: U512,
    ) -> U512 = crate::calculate_royalty;

}
