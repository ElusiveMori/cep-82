pub mod cep78 {
    //! Interactions with the CEP78 contract.

    use crate::named_arg;
    use alloc::vec;
    use casper_contract::contract_api::runtime;
    use casper_types::{ContractPackageHash, Key};

    use crate::TokenIdentifier;

    pub fn transfer(
        package: ContractPackageHash,
        token_id: TokenIdentifier,
        source_key: Key,
        target_key: Key,
    ) -> () {
        runtime::call_versioned_contract::<()>(
            package,
            None,
            "transfer",
            vec![
                token_id.to_named_arg(),
                named_arg!(source_key),
                named_arg!(target_key),
            ]
            .into(),
        )
    }
}

pub mod erc20 {
    use alloc::vec;
    use casper_contract::contract_api::runtime;
    use casper_types::{ContractPackageHash, Key, U256};

    use crate::named_arg;

    pub fn transfer_from(package: ContractPackageHash, owner: Key, recipient: Key, amount: U256) {
        runtime::call_versioned_contract::<()>(
            package,
            None,
            "transfer",
            vec![named_arg!(owner), named_arg!(recipient), named_arg!(amount)].into(),
        );
    }
}
