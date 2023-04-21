pub mod cep78 {
    use crate::token::TokenIdentifier;
    use crate::{named_arg, trace_block};
    use alloc::{string::String, vec};
    use casper_contract::contract_api::runtime;
    use casper_types::{ContractPackageHash, Key};

    pub fn transfer(
        package: ContractPackageHash,
        token_id: &TokenIdentifier,
        source_key: Key,
        target_key: Key,
    ) -> () {
        trace_block! {{
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
            );
        }}
    }

    pub fn metadata(package: ContractPackageHash, token_id: TokenIdentifier) -> String {
        trace_block! {{
            runtime::call_versioned_contract::<String>(
                package,
                None,
                "metadata",
                vec![token_id.to_named_arg()].into(),
            )
        }}
    }

    pub fn owner_of(package: ContractPackageHash, token_id: &TokenIdentifier) -> Key {
        trace_block! {{
            runtime::call_versioned_contract::<Key>(
                package,
                None,
                "owner_of",
                vec![token_id.to_named_arg()].into(),
            )
        }}
    }
}

pub mod erc20 {
    use alloc::vec;
    use casper_contract::contract_api::runtime;
    use casper_types::{ContractPackageHash, Key, U256};

    use crate::{named_arg, trace_block};

    pub fn approve(package: ContractPackageHash, spender: Key, amount: U256) {
        trace_block! {{
            runtime::call_versioned_contract::<()>(
                package,
                None,
                "approve",
                vec![named_arg!(spender), named_arg!(amount)].into(),
            );
        }}
    }

    pub fn transfer_from(package: ContractPackageHash, owner: Key, recipient: Key, amount: U256) {
        trace_block! {{
            runtime::call_versioned_contract::<()>(
                package,
                None,
                "transfer_from",
                vec![named_arg!(owner), named_arg!(recipient), named_arg!(amount)].into(),
            );
        }}
    }

    pub fn transfer(package: ContractPackageHash, recipient: Key, amount: U256) {
        trace_block! {{
            runtime::call_versioned_contract::<()>(
                package,
                None,
                "transfer",
                vec![named_arg!(recipient), named_arg!(amount)].into(),
            );
        }}
    }

    pub fn balance_of(package: ContractPackageHash, address: Key) -> U256 {
        trace_block! {{
            runtime::call_versioned_contract::<U256>(
                package,
                None,
                "balance_of",
                vec![named_arg!(address)].into(),
            )
        }}
    }
}

pub mod cep82 {
    pub mod custodial {
        use alloc::vec;
        use casper_contract::contract_api::runtime;
        use casper_types::{ContractPackageHash, Key, U256};

        use crate::{named_arg, token::TokenIdentifier, trace_block};

        pub fn delegate(
            package: ContractPackageHash,
            token_id: &TokenIdentifier,
        ) -> Option<ContractPackageHash> {
            trace_block! {{
                runtime::call_versioned_contract::<Option<ContractPackageHash>>(
                    package,
                    None,
                    "delegate",
                    vec![token_id.to_named_arg()].into(),
                )
            }}
        }

        pub fn transfer(
            package: ContractPackageHash,
            token_id: &TokenIdentifier,
            source_key: Key,
            target_key: Key,
            payment_source_key: Key,
            payment_token: ContractPackageHash,
            payment_amount: U256,
        ) -> () {
            trace_block! {{
                runtime::call_versioned_contract::<()>(
                    package,
                    None,
                    "transfer",
                    vec![
                        token_id.to_named_arg(),
                        named_arg!(source_key),
                        named_arg!(target_key),
                        named_arg!(payment_source_key),
                        named_arg!(payment_token),
                        named_arg!(payment_amount),
                    ]
                    .into(),
                )
            }}
        }

        pub fn calculate_royalty(
            package: ContractPackageHash,
            token_id: &TokenIdentifier,
            payment_token: ContractPackageHash,
            payment_amount: U256,
        ) -> U256 {
            trace_block! {{
                runtime::call_versioned_contract::<U256>(
                    package,
                    None,
                    "calculate_royalty",
                    vec![
                        token_id.to_named_arg(),
                        named_arg!(payment_token),
                        named_arg!(payment_amount),
                    ].into(),
                )
            }}
        }
    }
}
