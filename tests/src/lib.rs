#![cfg(test)]

pub mod util;

use casper_types::{U256, U512};
use util::{
    deploy::{deploy_cep78, deploy_cep82_custodial, deploy_cep82_marketplace, deploy_erc20},
    *,
};

use crate::util::state::RoyaltyStep;

// Test for basic functionality
#[test]
fn health_test() {
    let mut context = setup_context();
    let account = context.account.address;
    let (_, cep82_cs_package) = deploy_cep82_custodial(
        &mut context.builder,
        account,
        vec![],
        RoyaltyStep::basic(),
        account.into(),
    );
    let (cep78_hash, cep78_package) = deploy_cep78(
        &mut context.builder,
        context.account.address,
        Some(cep82_cs_package.into()),
    );
    let (cep82_mp_hash, _) =
        deploy_cep82_marketplace(&mut context.builder, context.account.address);

    cep78::register_owner(&mut context, cep78_hash, account.into());
    cep82::marketplace::register_nft(
        &mut context,
        cep82_mp_hash,
        cep78_package,
        Some(cep82_cs_package),
    );

    let (_, _, id1) = cep78::mint(&mut context, cep78_hash, account.into());
    let (_, _, id2) = cep78::mint(&mut context, cep78_hash, account.into());

    let owner1 = cep78::owner_of(&mut context, cep78_hash, id1);
    let owner2 = cep78::owner_of(&mut context, cep78_hash, id2);

    assert_eq!(owner1, owner2);
    assert_eq!(owner1, account.into());
}

#[test]
fn post_and_trade() {
    let mut context = setup_context();

    let manager = UserAccount::unique_account(&mut context, 10);
    let buyer = UserAccount::unique_account(&mut context, 20);

    let (cep82_cs_hash, cep82_cs_package) = deploy_cep82_custodial(
        &mut context.builder,
        context.account.address,
        vec![],
        RoyaltyStep::basic(),
        manager.address.into(),
    );

    let (cep78_hash, cep78_package) = deploy_cep78(
        &mut context.builder,
        context.account.address,
        Some(cep82_cs_hash.into()),
    );

    let (cep82_mp_hash, _) =
        deploy_cep82_marketplace(&mut context.builder, context.account.address);

    cep78::register_owner(&mut context, cep78_hash, buyer.address.into());
    cep78::register_owner(&mut context, cep78_hash, manager.address.into());
    cep82::marketplace::register_nft(
        &mut context,
        cep82_mp_hash,
        cep78_package,
        Some(cep82_cs_package),
    );

    let (_, _, id1) = cep78::mint(&mut context, cep78_hash, manager.address.into());

    let post_purse = new_purse(&mut context, manager.address, "post_purse", U512::from(0));

    set_current_sender(manager.address);
    cep78::approve(&mut context, cep78_hash, id1, cep82_mp_hash.into());
    let post_id = cep82::marketplace::post(
        &mut context,
        cep82_mp_hash,
        cep78_package,
        id1,
        1_000_000.into(),
        post_purse,
    );

    // let root_balance_before = erc20::balance_of(&mut context, erc20_hash, root.into());
    dbg!(manager.address);
    dbg!(buyer.address);
    set_current_sender(buyer.address);
    let bid_purse = new_purse(
        &mut context,
        buyer.address,
        "bid_purse",
        U512::from(10_000_000),
    );
    cep82::marketplace::bid(
        &mut context,
        cep82_mp_hash,
        post_id,
        bid_purse,
        1_000_000.into(),
    );

    let owner = cep78::owner_of(&mut context, cep78_hash, id1);
    assert_eq!(owner, buyer.address.into());
}
