#!/usr/bin/env bash

(
    cd contract_marketplace
    cargo build --release --target wasm32-unknown-unknown --features onchain,test-support
)

(
    cd contract_custodial
    cargo build --release --target wasm32-unknown-unknown --features onchain,test-support
)

(
    cd contract_testutil
    cargo build --release --target wasm32-unknown-unknown --features onchain,test-support
    cp target/wasm32-unknown-unknown/release/testutil.wasm ../wasm/testutil.wasm
)

