# CEP-82 - Mandatory Royalties for CEP-78

## Overview

CEP-82 is a proposal which aims to create a standard for enforced royalties in the Casper ecosystem for CEP-78 compatible NFTs, as well as the interactions between marketplaces that support this enforcement and the other pieces of the involved machinery.

## Rationale

It is desirable to be able to enforce royalty collection to various degrees, and to standardize the exchange protocol that would facilitate such enforcement, as otherwise it is trivial for a third party to bypass the royalty fees and to leave the artist completely out of the loop.

## Mechanism

This proposal uses the new CEP-78 `can_transfer` transfer filter functionality. It is used to install an auxiliary "custodial" contract on top of CEP-78, which ensures that royalty collection is performed before an NFT can be transferred between parties.

The protocol is as follows:
* The marketplace wishes to initiate a sale of an NFT from User A to User B,
* The marketplace calls the `calculate_royalty` method on the custodial contract, passing in the value of the sale, and acquires the royalty size it must pay to the custodial contract in order for the transfer to be authorized,
* Under the hood, the custodial contract may use whatever logic to determine this royalty size. It may be a percentage fee, a flat fee, with an option for a minimum, or some more complex tiered scheme. The provided sample implementation in `contract_custodial` offers a flat fee, a percentage fee, both with the option of adding a minimum fee size.
* The marketplace deposits the royalty to be collected in a separate purse, and then calls `pay_royalty` on the custodial contract with that purse. If everything checks out, the custodial contract transfers the royalty internally to a purse that it owns exclusively, and marks the NFT as pending a transfer.
* The marketplace can then call the `transfer` method as usual. The paid royalty marker will allow the transfer to go through.

As an additional measure, a custodial contract may opt into whitelisting only specific, known-good marketplaces to avoid an arbitrary marketplaces avoiding a percentage-based fee by misreporting the sale value. This is not an issue in the flat-fee scenario, however, since it is required for the royalty to be paid before a transfer is permitted.

## Implementation

In this repository are two core contracts that demonstrate this interaction:
* `contract_custodial` implements a "custodial" contract that implements the `can_transfer` interface for a CEP-78 contract, as well as the interface for marketplace.
* `contract_marketplace` implements a simple post board type marketplace that supports the aforementioned custodial contract to demonstrate the interaction. For the purposes of royalty enforcement, however, the particular interface and semantics of the marketplace are unimportant - any number of marketplaces types could be implemented instead of it. What is important is the way this marketplace contract interacts with the custodial contract.

In principle, however, nothing demands that these implementations are used as-is or become standard. It is likely that many different royalty tiering schemes might be desirable, as well as different marketplace implementations.