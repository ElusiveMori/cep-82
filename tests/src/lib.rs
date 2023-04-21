#![cfg(test)]

pub mod util;

use casper_types::U256;
use util::{
    deploy::{deploy_cep78, deploy_cep82_custodial, deploy_cep82_marketplace, deploy_erc20},
    *,
};

// Test for basic functionality
#[test]
fn health_test() {
    let mut context = setup_context();
    let account = context.account.address;
    let (_, erc20_package) = deploy_erc20(&mut context.builder, context.account.address);
    let (cep78_hash, cep78_package) = deploy_cep78(&mut context.builder, context.account.address);
    let (cep82_hash, _) = deploy_cep82_marketplace(&mut context.builder, context.account.address);

    cep78::register_owner(&mut context, cep78_hash, account.into());
    cep82::marketplace::register_erc(&mut context, cep82_hash, erc20_package);
    cep82::marketplace::register_nft(&mut context, cep82_hash, cep78_package, false);

    let (_, _, id1) = cep78::mint(&mut context, cep78_hash, account.into());
    let (_, _, id2) = cep78::mint(&mut context, cep78_hash, account.into());

    let owner1 = cep78::owner_of(&mut context, cep78_hash, id1);
    let owner2 = cep78::owner_of(&mut context, cep78_hash, id2);

    assert_eq!(owner1, owner2);
    assert_eq!(owner1, account.into());
}

#[test]
fn wrap_and_trade() {
    let mut context = setup_context();
    let root = context.account.address;

    let manager = UserAccount::unique_account(&mut context, 10);
    let buyer = UserAccount::unique_account(&mut context, 20);

    let (erc20_hash, erc20_package) = deploy_erc20(&mut context.builder, context.account.address);
    let (cep78_hash, cep78_package) = deploy_cep78(&mut context.builder, manager.address);
    let (cep82_market_hash, cep82_market_package) =
        deploy_cep82_marketplace(&mut context.builder, context.account.address);

    erc20::transfer_from(
        &mut context,
        erc20_hash,
        root.into(),
        buyer.key(),
        10_000_000.into(),
    );

    let (cep82_custodial_hash, cep82_custodial_package) = deploy_cep82_custodial(
        &mut context.builder,
        manager.address,
        cep78_package,
        vec![erc20_package],
        vec![cep82_market_package],
        U256::from(10),
        manager.address.into(),
    );

    cep78::register_owner(&mut context, cep78_hash, cep82_custodial_package.into());

    set_current_sender(manager.address);
    let (_, _, token_id) = cep82::custodial::mint_and_claim(
        &mut context,
        cep78_hash,
        cep82_custodial_hash,
        cep82_custodial_package,
        root.into(),
    );
    set_current_sender(None);

    let owner = cep78::owner_of(&mut context, cep78_hash, token_id);
    let custodial_owner = cep78::owner_of(&mut context, cep82_custodial_hash, token_id);

    assert_eq!(owner, cep82_custodial_package.into());
    assert_eq!(custodial_owner, root.into());

    cep82::marketplace::register_erc(&mut context, cep82_market_hash, erc20_package);
    cep82::marketplace::register_nft(
        &mut context,
        cep82_market_hash,
        cep82_custodial_package,
        true,
    );

    cep82::custodial::set_delegate(
        &mut context,
        cep82_custodial_hash,
        token_id,
        Some(cep82_market_package),
    );

    let post_id = cep82::marketplace::post(
        &mut context,
        cep82_market_hash,
        cep82_custodial_package,
        token_id,
        1_000_000.into(),
        erc20_package,
    );

    let root_balance_before = erc20::balance_of(&mut context, erc20_hash, root.into());
    set_current_sender(buyer.address);
    erc20::approve(
        &mut context,
        erc20_hash,
        cep82_market_package.into(),
        1_000_000.into(),
    );
    cep82::marketplace::bid(&mut context, cep82_market_hash, post_id, 1_000_000.into());
    set_current_sender(None);

    let owner = cep78::owner_of(&mut context, cep82_custodial_hash, token_id);
    assert_eq!(owner, buyer.address.into());
    let manager_balance = erc20::balance_of(&mut context, erc20_hash, manager.address.into());
    assert_eq!(manager_balance, 1_000.into());
    let buyer_balance = erc20::balance_of(&mut context, erc20_hash, buyer.address.into());
    assert_eq!(buyer_balance, 9_000_000.into());
    let root_balance = erc20::balance_of(&mut context, erc20_hash, root.into());
    assert_eq!(root_balance - root_balance_before, 999_000.into());

    dbg!(post_id);
}
