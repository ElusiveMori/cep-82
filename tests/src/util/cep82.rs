use casper_types::{runtime_args, ContractHash, ContractPackageHash, RuntimeArgs, U256};

use super::{call_contract, TestContext};

pub mod marketplace {
    use casper_types::URef;

    use crate::util::call_contract_with_result;

    use super::*;

    pub fn register_nft(
        context: &mut TestContext,
        contract: ContractHash,
        nft_package: ContractPackageHash,
        custodial_package: Option<ContractPackageHash>,
    ) {
        let mut args = RuntimeArgs::new();
        args.insert("nft_package", nft_package).unwrap();
        if let Some(custodial_package) = custodial_package {
            args.insert("custodial_package", custodial_package).unwrap();
        }

        call_contract(context, contract, "register_cep78_contract", args)
    }

    pub fn post(
        context: &mut TestContext,
        contract: ContractHash,
        nft_contract: ContractPackageHash,
        token_id: u64,
        price: U256,
        target_purse: URef,
    ) -> u64 {
        call_contract_with_result::<u64>(
            context,
            contract,
            "post",
            runtime_args! {
                "nft_contract" => nft_contract,
                "token_id" => token_id,
                "price" => price,
                "target_purse" => target_purse,
            },
        )
    }

    pub fn bid(
        context: &mut TestContext,
        contract: ContractHash,
        post_id: u64,
        source_purse: URef,
        amount: U256,
    ) {
        call_contract(
            context,
            contract,
            "bid",
            runtime_args! {
                "post_id" => post_id,
                "source_purse" => source_purse,
                "amount" => amount,
            },
        )
    }
}

pub mod custodial {
    use casper_types::{runtime_args, ContractHash, Key};
    use casper_types::{ContractPackageHash, RuntimeArgs};

    use crate::util::{call_contract, call_contract_with_result, cep78, TestContext};

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
        call_contract_with_result::<Option<ContractPackageHash>>(
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
