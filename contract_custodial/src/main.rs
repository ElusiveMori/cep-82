#![no_std]
#![no_main]

use contract_common::forward_entrypoints;
#[allow(unused)]
use contract_custodial::entry_point as ep;

forward_entrypoints! {
    ep: [ call ]
}

forward_entrypoints! {
    ep: [
        transfer,
        balance_of,
        owner_of,
        metadata,
        delegate,
        set_delegate,
        claim,
        calculate_royalty,
    ]
}
