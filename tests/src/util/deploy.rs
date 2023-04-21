use casper_engine_test_support::{
    ExecuteRequestBuilder, WasmTestBuilder, ARG_AMOUNT, DEFAULT_PAYMENT,
};

use casper_execution_engine::{
    core::{engine_state::Error as EngineError, execution::Error as ExecError},
    storage::global_state::{CommitProvider, StateProvider},
};

use super::{
    cep78::{InstallerRequestBuilder, MetadataMutability, OwnershipMode},
    deploy_builder, CONTRACT_CEP78_BYTES, CONTRACT_CEP82_CUSTODIAL_BYTES,
    CONTRACT_CEP82_MARKETPLACE_BYTES, CONTRACT_ERC20_BYTES,
};
use casper_types::{
    account::AccountHash, runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};

pub fn deploy_contract<S>(
    builder: &mut WasmTestBuilder<S>,
    account: AccountHash,
    wasm_bytes: &[u8],
    deploy_args: RuntimeArgs,
    contract_key: &str,
) -> (ContractHash, ContractPackageHash)
where
    S: StateProvider + CommitProvider,
    EngineError: From<S::Error>,
    <S as StateProvider>::Error: Into<ExecError>,
{
    let deploy_item = deploy_builder()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_session_bytes(wasm_bytes.into(), deploy_args)
        .with_authorization_keys(&[account])
        .with_address(account)
        .build();

    let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

    builder.exec(execute_request).commit().expect_success();

    let stored_account = builder.query(None, Key::Account(account), &[]).unwrap();

    let contract_hash = stored_account
        .as_account()
        .unwrap()
        .named_keys()
        .get(contract_key)
        .unwrap()
        .into_hash()
        .unwrap();

    let contract_package_hash = builder
        .query(None, Key::Hash(contract_hash), &[])
        .unwrap()
        .as_contract()
        .unwrap()
        .contract_package_hash();

    (ContractHash::new(contract_hash), contract_package_hash)
}

pub fn deploy_erc20<S>(
    builder: &mut WasmTestBuilder<S>,
    account: AccountHash,
) -> (ContractHash, ContractPackageHash)
where
    S: StateProvider + CommitProvider,
    EngineError: From<S::Error>,
    <S as StateProvider>::Error: Into<ExecError>,
{
    let deploy_args = runtime_args! {
        "name" => "test token".to_string(),
        "symbol" => "TTKN",
        "decimals" => 9u8,
        "total_supply" => U256::max_value(),
    };

    deploy_contract(
        builder,
        account,
        CONTRACT_ERC20_BYTES,
        deploy_args,
        "erc20_token_contract",
    )
}

pub fn deploy_cep78<S>(
    builder: &mut WasmTestBuilder<S>,
    account: AccountHash,
) -> (ContractHash, ContractPackageHash)
where
    S: StateProvider + CommitProvider,
    EngineError: From<S::Error>,
    <S as StateProvider>::Error: Into<ExecError>,
{
    let deploy_args = InstallerRequestBuilder::default()
        .with_total_token_supply(100u64)
        .with_ownership_mode(OwnershipMode::Transferable)
        .with_identifier_mode(super::cep78::NFTIdentifierMode::Ordinal)
        .with_metadata_mutability(MetadataMutability::Mutable)
        .build();

    deploy_contract(
        builder,
        account,
        CONTRACT_CEP78_BYTES,
        deploy_args,
        "cep78_contract_hash_nft-test",
    )
}

pub fn deploy_cep82_marketplace<S>(
    builder: &mut WasmTestBuilder<S>,
    account: AccountHash,
) -> (ContractHash, ContractPackageHash)
where
    S: StateProvider + CommitProvider,
    EngineError: From<S::Error>,
    <S as StateProvider>::Error: Into<ExecError>,
{
    let deploy_args = runtime_args! {};

    deploy_contract(
        builder,
        account,
        CONTRACT_CEP82_MARKETPLACE_BYTES,
        deploy_args,
        "cep82_marketplace",
    )
}

pub fn deploy_cep82_custodial<S>(
    builder: &mut WasmTestBuilder<S>,
    account: AccountHash,
    wrapped_contract: ContractPackageHash,
    whitelisted_payment_tokens: Vec<ContractPackageHash>,
    whitelisted_marketplaces: Vec<ContractPackageHash>,
    royalty_percent: U256,
    manager: Key,
) -> (ContractHash, ContractPackageHash)
where
    S: StateProvider + CommitProvider,
    EngineError: From<S::Error>,
    <S as StateProvider>::Error: Into<ExecError>,
{
    let deploy_args = runtime_args! {
        "wrapped_contract" => wrapped_contract,
        "whitelisted_payment_tokens" => whitelisted_payment_tokens,
        "whitelisted_marketplaces" => whitelisted_marketplaces,
        "royalty_percent" => royalty_percent,
        "manager" => manager,
    };

    deploy_contract(
        builder,
        account,
        CONTRACT_CEP82_CUSTODIAL_BYTES,
        deploy_args,
        "cep82_custodial",
    )
}
