#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::{
    runtime::{self},
    storage,
};
use casper_types::{
    bytesrepr::{self, Bytes, FromBytes},
    ContractHash, RuntimeArgs,
};
use casper_types::{CLType, CLTyped};

// Simple wrapper for a contract call that memoizes the result in the caller's
// context so that it can be retrieved later.
#[no_mangle]
pub extern "C" fn call() {
    let target = runtime::get_named_arg::<ContractHash>("target");
    let ep = runtime::get_named_arg::<String>("entry_point_name");
    let args = runtime::get_named_arg::<Bytes>("args");
    let (args, _) = RuntimeArgs::from_bytes(&args).unwrap();
    let data = runtime::call_contract::<AllBytes>(target, &ep, args);
    let uref = storage::new_uref(Bytes::from(data.data));
    runtime::put_key("__result", uref.into());
}

struct AllBytes {
    data: Vec<u8>,
}

impl FromBytes for AllBytes {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        Ok((
            Self {
                data: bytes.to_vec(),
            },
            &[],
        ))
    }
}

impl CLTyped for AllBytes {
    fn cl_type() -> CLType {
        CLType::Any
    }
}
