use std::{
    iter::repeat,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use base64::Engine;
use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH,
    DEFAULT_PAYMENT,
};

use casper_execution_engine::{
    core::{
        engine_state::{
            self, run_genesis_request::RunGenesisRequest, DeployItem, Error as EngineError,
            GenesisAccount,
        },
        execution::{self, Error as ExecError},
    },
    storage::global_state::{in_memory::InMemoryGlobalState, CommitProvider, StateProvider},
};

use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes, ToBytes},
    runtime_args, ApiError, CLValue, ContractHash, Key, Motes, PublicKey, RuntimeArgs, SecretKey,
    StoredValue, URef, U256, U512,
};
use once_cell::sync::Lazy;

pub mod cep78;
pub mod cep82;
pub mod deploy;
pub mod erc20;
pub mod state;

const TEST_ACCOUNT_BALANCE: u64 = 10_000_000_000_000u64;
const TEST_ACCOUNT: [u8; 32] = [255u8; 32];
const CONTRACT_ERC20_BYTES: &[u8] = include_bytes!("../../wasm/erc20.wasm");
const CONTRACT_CEP78_BYTES: &[u8] = include_bytes!("../../wasm/cep78.wasm");
const CONTRACT_CEP82_MARKETPLACE_BYTES: &[u8] = include_bytes!("../../wasm/cep82-marketplace.wasm");
const CONTRACT_CEP82_CUSTODIAL_BYTES: &[u8] = include_bytes!("../../wasm/cep82-custodial.wasm");
const CONTRACT_TESTUTIL_BYTES: &[u8] = include_bytes!("../../wasm/testutil.wasm");
static DEPLOY_COUNTER: AtomicUsize = AtomicUsize::new(0);

static CURRENT_SENDER: Lazy<Mutex<Option<AccountHash>>> = Lazy::new(|| Mutex::new(None));

pub fn new_deploy_hash() -> [u8; 32] {
    let counter = DEPLOY_COUNTER.fetch_add(1, Ordering::SeqCst);
    let hash = repeat(counter)
        .take(4)
        .flat_map(|i| i.to_le_bytes())
        .collect::<Vec<_>>();
    hash.try_into().unwrap()
}

pub fn deploy_builder() -> DeployItemBuilder {
    DeployItemBuilder::new().with_deploy_hash(new_deploy_hash())
}

pub struct TestContext {
    pub account: UserAccount,
    pub builder: InMemoryWasmTestBuilder,
}

pub struct UserAccount {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub address: AccountHash,
}

impl UserAccount {
    fn new(secret_key: SecretKey) -> Self {
        let public_key = PublicKey::from(&secret_key);
        let address = AccountHash::from(&public_key);
        Self {
            secret_key,
            public_key,
            address,
        }
    }

    pub fn unique_account(context: &mut TestContext, unique_id: u8) -> Self {
        if unique_id == 255 {
            panic!("Account with id 255 booked for genesis account");
        }
        // Create a key using unique_id
        let secret_key = SecretKey::ed25519_from_bytes([unique_id; 32]).unwrap();
        let account = UserAccount::new(secret_key);

        // We need to transfer some funds to the account so it become active
        let deploy = simple_deploy_builder(context.account.address)
            .with_transfer_args(runtime_args![
                ARG_AMOUNT => U512::one() * TEST_ACCOUNT_BALANCE,
                "target" => account.public_key.clone(),
                "id" => Some(u64::from(unique_id))
            ])
            .build();
        context
            .builder
            .exec(ExecuteRequestBuilder::from_deploy_item(deploy).build())
            .commit()
            .expect_success();
        account
    }

    pub fn key(&self) -> Key {
        Key::from(self.address)
    }
}

pub fn setup_context() -> TestContext {
    // Create keypair.
    let secret_key = SecretKey::ed25519_from_bytes(TEST_ACCOUNT).unwrap();
    let account_data = UserAccount::new(secret_key);

    // Create a GenesisAccount.
    let account = GenesisAccount::account(
        account_data.public_key.clone(),
        Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
        None,
    );

    let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
    genesis_config.ee_config_mut().push_account(account);

    let run_genesis_request = RunGenesisRequest::new(
        *DEFAULT_GENESIS_CONFIG_HASH,
        genesis_config.protocol_version(),
        genesis_config.take_ee_config(),
    );

    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&run_genesis_request).commit();

    TestContext {
        account: account_data,
        builder,
    }
}

pub fn simple_deploy_builder(account: AccountHash) -> DeployItemBuilder {
    deploy_builder()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_authorization_keys(&[account])
        .with_address(account)
}

pub fn dictionary_key<T: ToBytes>(value: &T) -> String {
    base64::prelude::BASE64_STANDARD_NO_PAD.encode(value.to_bytes().unwrap())
}

pub fn query_balance<S>(
    builder: &mut WasmTestBuilder<S>,
    contract: ContractHash,
    address: &Key,
) -> U256
where
    S: StateProvider + CommitProvider,
    EngineError: From<S::Error>,
    <S as StateProvider>::Error: Into<ExecError>,
{
    let contract = builder
        .query(None, Key::Hash(contract.value()), &[])
        .unwrap()
        .as_contract()
        .cloned()
        .unwrap();

    let balance_uref = contract
        .named_keys()
        .get("balances")
        .unwrap()
        .as_uref()
        .cloned()
        .unwrap();

    let balance = builder
        .query_dictionary_item(None, balance_uref, &dictionary_key(address))
        .unwrap()
        .as_cl_value()
        .cloned()
        .unwrap()
        .into_t::<U256>()
        .unwrap();

    balance
}

pub fn arbitrary_user(context: &mut TestContext) -> UserAccount {
    UserAccount::unique_account(context, 0)
}

pub fn arbitrary_user_key(context: &mut TestContext) -> Key {
    arbitrary_user(context).key()
}

pub fn execution_context(
    context: &mut TestContext,
    deploy_item: DeployItem,
) -> &mut WasmTestBuilder<InMemoryGlobalState> {
    context
        .builder
        .exec(ExecuteRequestBuilder::from_deploy_item(deploy_item).build())
        .commit()
}

pub fn execution_error(context: &mut TestContext, deploy_item: DeployItem) -> EngineError {
    execution_context(context, deploy_item)
        .expect_failure()
        .get_error()
        .unwrap()
}

pub fn set_current_sender<T: Into<Option<AccountHash>>>(account: T) {
    let mut current_sender = CURRENT_SENDER.lock().unwrap();
    *current_sender = account.into();
}

pub fn current_sender() -> Option<AccountHash> {
    let current_sender = CURRENT_SENDER.lock().unwrap();
    *current_sender
}

pub fn call_contract_with_result<T: FromBytes>(
    context: &mut TestContext,
    contract: ContractHash,
    entry_point: &str,
    args: RuntimeArgs,
) -> T {
    let sender = current_sender().unwrap_or(context.account.address);
    let mut runtime_args = RuntimeArgs::new();
    runtime_args.insert("action", "call").unwrap();
    runtime_args.insert("target", contract).unwrap();
    runtime_args
        .insert("entry_point_name", entry_point)
        .unwrap();

    runtime_args.insert_cl_value(
        "args",
        CLValue::from_t(Bytes::from(args.to_bytes().unwrap())).unwrap(),
    );

    let call_request =
        ExecuteRequestBuilder::module_bytes(sender, CONTRACT_TESTUTIL_BYTES.to_vec(), runtime_args)
            .build();

    context
        .builder
        .exec(call_request)
        .commit()
        .expect_success_ex();
    let returned = context
        .builder
        .query(None, Key::Account(sender), &["__result".to_string()])
        .unwrap();

    let value = match returned {
        StoredValue::CLValue(cl_value) => cl_value,
        _ => panic!("Expected CLValue"),
    };

    let data: Bytes = value.into_t().unwrap();
    let (result, _) = T::from_bytes(&data).unwrap();

    result
}

pub fn call_contract(
    context: &mut TestContext,
    contract: ContractHash,
    entry_point: &str,
    args: RuntimeArgs,
) {
    let call_request = ExecuteRequestBuilder::contract_call_by_hash(
        current_sender().unwrap_or(context.account.address),
        contract,
        entry_point,
        args,
    )
    .build();

    context
        .builder
        .exec(call_request)
        .commit()
        .expect_success_ex();
}

pub fn new_purse(
    context: &mut TestContext,
    account: AccountHash,
    name: &str,
    amount: U512,
) -> URef {
    let mut runtime_args = RuntimeArgs::new();
    runtime_args.insert("action", "new_purse").unwrap();
    runtime_args.insert("name", name).unwrap();
    runtime_args.insert("amount", amount).unwrap();

    let call_request = ExecuteRequestBuilder::module_bytes(
        account,
        CONTRACT_TESTUTIL_BYTES.to_vec(),
        runtime_args,
    )
    .build();

    context
        .builder
        .exec(call_request)
        .commit()
        .expect_success_ex();

    let returned = context.builder.get_account(account).unwrap().named_keys()[name];

    match returned {
        Key::URef(uref) => uref,
        _ => panic!("Expected URef"),
    }
}

trait TestBuilderExt {
    fn expect_success_ex(&mut self) -> &mut Self;
}

impl TestBuilderExt for WasmTestBuilder<InMemoryGlobalState> {
    fn expect_success_ex(&mut self) -> &mut Self {
        let exec_results = self
            .get_exec_results()
            .last()
            .expect("Expected to be called after run()");
        let exec_result = exec_results
            .get(0)
            .expect("Unable to get first deploy result");

        if exec_result.is_failure() {
            if let Some(engine_state::Error::Exec(execution::Error::Revert(ApiError::User(code)))) =
                exec_result.as_error()
            {
                let error = erc20::Error::from(*code);

                if error != erc20::Error::Unknown {
                    eprintln!("Possible ERC20 error: {error:?}");
                }
            }

            panic!(
                "Expected successful execution result, but instead got: {:#?}",
                exec_result.as_error().unwrap(),
            );
        }
        self
    }
}
