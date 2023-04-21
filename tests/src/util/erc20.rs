use casper_types::{account::AccountHash, runtime_args, ContractHash, U256};
use casper_types::{Key, RuntimeArgs};

use super::{call_contract, call_contract_memoized, TestContext};

pub fn balance_of(context: &mut TestContext, contract: ContractHash, address: Key) -> U256 {
    call_contract_memoized::<U256>(
        context,
        contract,
        "balance_of",
        runtime_args! {
            "address" => address,
        },
    )
}

pub fn transfer_from(
    context: &mut TestContext,
    contract: ContractHash,
    owner: Key,
    recipient: Key,
    amount: U256,
) {
    call_contract(
        context,
        contract,
        "transfer",
        runtime_args! {
            "owner" => owner,
            "recipient" => recipient,
            "amount" => amount,
        },
    )
}

pub fn approve(context: &mut TestContext, contract: ContractHash, spender: Key, amount: U256) {
    call_contract(
        context,
        contract,
        "approve",
        runtime_args! {
            "spender" => spender,
            "amount" => amount,
        },
    )
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    InvalidContext,
    InsufficientBalance,
    InsufficientAllowance,
    Overflow,
    Unknown,
}

const ERROR_INVALID_CONTEXT: u16 = u16::MAX;
const ERROR_INSUFFICIENT_BALANCE: u16 = u16::MAX - 1;
const ERROR_INSUFFICIENT_ALLOWANCE: u16 = u16::MAX - 2;
const ERROR_OVERFLOW: u16 = u16::MAX - 3;

impl From<u16> for Error {
    fn from(error: u16) -> Self {
        match error {
            ERROR_INVALID_CONTEXT => Error::InvalidContext,
            ERROR_INSUFFICIENT_BALANCE => Error::InsufficientBalance,
            ERROR_INSUFFICIENT_ALLOWANCE => Error::InsufficientAllowance,
            ERROR_OVERFLOW => Error::Overflow,
            _ => Error::Unknown,
        }
    }
}
