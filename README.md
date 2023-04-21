# CEP-82 - Mandatory Royalties for CEP-78

This repository contains a proposed implementation of CEP-82, which aims to create a standard for enforced royalties in the Casper ecosystem for CEP-78 compatible NFTs, as well as the interactions between marketplaces that support this enforcement and the wrapper NFT contract that does the actual enforcement.

In this repository are two core contracts that demonstrate this interaction:
* `contract_custodial` implements a "custodial" contract that wraps a CEP-78 contract and exposes a semi-compatible CEP-78 interface to consumers.
* `contract_marketplace` implements a simple post board type marketplace that supports the aforementioned custodial contract to demonstrate the interaction. For the purposes of royalty enforcement, however, the particular interface and semantics of the marketplace are unimportant - any number of marketplaces types could be implemented instead of it. What is important is the way this marketplace contract interacts with the custodial contract.

## Royalty enforcement mechanism

There are two core mechanisms here that together allow to enforce collection of royalties:

* It is mandatory to report the transfer value to a CEP-82 wrapped NFT, which allows the CEP-82 custodial to calculate the royalty and transfer it to the collector,
* Only whitelisted marketplaces can invoke the transfer method on the custodial contract. The whitelist can only be managed by the NFT manager.

Notably, _some_ level of trust still has to be established between the NFT manager and a whitelisted marketplace. Particularly, it is expected that a marketplace correctly reports the sale value of an NFT to the custodial. In the absence of this trust, nothing guarantees that a compliant marketplace doesn't simply misreport the sale value to the custodial, although this could be partially mitigated by e.g. enforcing a minimal royalty fee. 

The usage of a whitelist enforces that the NFT can at least only be sold on known-good marketplaces, and that the royalty collection mechanism can't be fully bypassed.

### Restricted transfers

CEP-82 ensures that only whitelisted marketplaces can call the transfer method by hijacking ownership of the underlying NFTs, and re-implementing ownership logic on top, in particular the `balance_of` and `owner_of` querying methods. `metadata` is forwarded to the underlying contract for convenience. The idea is that the CEP-82 contract might adequately "masquerade" as the underlying CEP-78 contract to an outside observer, at least for the purposes of basic data querying. 

__NB:__ This approach is __suboptimal__, because it throws away the existing underlying infrastructure of CEP-78, such as eventing and ownership semantics. However, in the current CEP-78 implementation, there is no other way to enforce that an NFT holder doesn't bypass royalty collection. __A potential solution__ would require some minor changes in CEP-78 to support setting a callback contract to override `transfer` behaviour.

## Notes on current implementation

1. The custodial contract has to be made aware of NFTs by calling the `claim` method. Only the manager can call this method. See tests for implementation details.
2. Current implementation focuses on demonstrating the custodial mechanism, rather than trying to offer a battle-tested and production-ready product.

## Open questions:

1. The biggest question is whether or not CEP-78 should be modified to support restricting `transfer` logic, or CEP-82 expanded to re-implement the missing logic (eventing, etc.)
2. Whether more marketplace implementations should be a part of CEP-82, or if CEP-82 should only focus on defining the interaction between marketplaces, NFTs, and royalty collectors.
