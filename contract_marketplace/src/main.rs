#![no_std]
#![no_main]

use contract_common::forward_entrypoints;
#[allow(unused)]
use contract_marketplace::entry_point as ep;

forward_entrypoints! {
    ep: [ call ]
}

forward_entrypoints! {
    ep: [
        bid,
        post,
        cancel,
        register_erc20_contract,
        register_cep78_contract,
        request_undelegate,
    ]
}
