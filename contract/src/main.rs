#![no_std]
#![no_main]

extern crate casper_contract;

use contract::forward_entrypoints;

#[no_mangle]
extern "C" fn call() {
    contract::install();
}

use contract::entry_point as ep;
forward_entrypoints! {
    ep: [
        bid,
        unbid,
        post,
        cancel,
        get_real_owner,
    ]
}
