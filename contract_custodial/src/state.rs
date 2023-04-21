use contract_common::{prelude::*, token::TokenIdentifier};

use crate::CustodialError;

serializable_structs! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OwnerData {
        pub owned_count: u64,
    }
}

named_keys! {
    init_all(manager: Key, wrapped_contract: ContractPackageHash, royalty_percent: U256):
    dict real_owner_by_token: Key;
    dict delegate_by_token: ContractPackageHash;
    dict owner_data: OwnerData;
    dict whitelisted_marketplaces: bool;
    dict whitelisted_payment_tokens: bool;

    val manager: Key = manager;
    val wrapped_contract: ContractPackageHash = wrapped_contract;
    val royalty_percent: U256 = royalty_percent;
}

pub fn check_is_token_known(token: &TokenIdentifier) {
    real_owner_of(token);
}

pub fn real_owner_of(token: &TokenIdentifier) -> Key {
    real_owner_by_token::try_read(&b64_cl(token))
        .unwrap_or_revert_with(CustodialError::UnknownToken)
}

pub fn try_real_owner_of(token: &TokenIdentifier) -> Option<Key> {
    real_owner_by_token::try_read(&b64_cl(token))
}

pub fn set_real_owner(token: &TokenIdentifier, owner: Key) {
    real_owner_by_token::write(&b64_cl(token), owner);
}

pub fn delegate_of(token: &TokenIdentifier) -> Option<ContractPackageHash> {
    delegate_by_token::try_read(&b64_cl(token))
}

pub fn set_delegate(token: &TokenIdentifier, delegate: Option<ContractPackageHash>) {
    if let Some(delegate) = delegate {
        delegate_by_token::write(&b64_cl(token), delegate);
    } else {
        delegate_by_token::remove(&b64_cl(token));
    }
}

pub fn is_payment_token_whitelisted(token: ContractPackageHash) -> bool {
    whitelisted_payment_tokens::try_read(&b64_cl(&token)).unwrap_or(false)
}

pub fn is_marketplace_whitelisted(marketplace: ContractPackageHash) -> bool {
    whitelisted_marketplaces::try_read(&b64_cl(&marketplace)).unwrap_or(false)
}
