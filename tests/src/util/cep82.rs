use casper_types::{runtime_args, ContractHash, ContractPackageHash, RuntimeArgs, U256};

use super::{call_contract, TestContext};

pub mod marketplace {
    use crate::util::call_contract_memoized;

    use super::*;

    pub fn register_nft(
        context: &mut TestContext,
        contract: ContractHash,
        to_register: ContractPackageHash,
        is_cep82_compliant: bool,
    ) {
        call_contract(
            context,
            contract,
            "register_cep78_contract",
            runtime_args! {
                "package" => to_register,
                "is_cep82_compliant" => is_cep82_compliant,
            },
        )
    }

    pub fn register_erc(
        context: &mut TestContext,
        contract: ContractHash,
        to_register: ContractPackageHash,
    ) {
        call_contract(
            context,
            contract,
            "register_erc20_contract",
            runtime_args! {
                "package" => to_register,
            },
        )
    }

    pub fn post(
        context: &mut TestContext,
        contract: ContractHash,
        nft_contract: ContractPackageHash,
        token_id: u64,
        price: U256,
        quote_token_contract: ContractPackageHash,
    ) -> u64 {
        call_contract_memoized::<u64>(
            context,
            contract,
            "post",
            runtime_args! {
                "nft_contract" => nft_contract,
                "token_id" => token_id,
                "price" => price,
                "quote_token_contract" => quote_token_contract,
            },
        )
    }

    pub fn bid(context: &mut TestContext, contract: ContractHash, post_id: u64, amount: U256) {
        call_contract(
            context,
            contract,
            "bid",
            runtime_args! {
                "post_id" => post_id,
                "amount" => amount,
            },
        )
    }
}

pub mod custodial {
    use casper_types::{runtime_args, ContractHash, Key};
    use casper_types::{ContractPackageHash, RuntimeArgs};

    use crate::util::{call_contract, call_contract_memoized, cep78, TestContext};

    pub fn claim(context: &mut TestContext, contract: ContractHash, token_id: u64, owner: Key) {
        call_contract(
            context,
            contract,
            "claim",
            runtime_args! {
                "token_id" => token_id,
                "owner" => owner,
            },
        )
    }

    pub fn set_delegate(
        context: &mut TestContext,
        contract: ContractHash,
        token_id: u64,
        delegate: Option<ContractPackageHash>,
    ) {
        let mut args = RuntimeArgs::new();
        args.insert("token_id", token_id).unwrap();
        if let Some(delegate) = delegate {
            args.insert("new_delegate", delegate).unwrap();
        }

        call_contract(context, contract, "set_delegate", args)
    }

    pub fn delegate(
        context: &mut TestContext,
        contract: ContractHash,
        token_id: u64,
    ) -> Option<ContractPackageHash> {
        call_contract_memoized::<Option<ContractPackageHash>>(
            context,
            contract,
            "delegate",
            runtime_args! {
                "token_id" => token_id,
            },
        )
    }

    pub fn mint_and_claim(
        context: &mut TestContext,
        nft_contract: ContractHash,
        custodial_contract: ContractHash,
        custodial_contract_package: ContractPackageHash,
        owner: Key,
    ) -> (String, Key, u64) {
        let (a, b, id) = cep78::mint(context, nft_contract, custodial_contract_package.into());
        claim(context, custodial_contract, id, owner);
        (a, b, id)
    }
}
