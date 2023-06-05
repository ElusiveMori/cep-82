//! Utilities for working with the contract call stack.
use casper_types::system::CallStackElement;
use once_cell::unsync::Lazy;

use crate::prelude::*;

st_non_sync_static! {
    Lazy<alloc::vec::Vec<CallStackElement>> = Lazy::new(|| {
        runtime::get_call_stack()
    })
}

/// Equivalent of [`runtime::get_call_stack`], but cached across invocations.
pub fn read() -> &'static [CallStackElement] {
    get()
}

/// Get a call stack element at `depth`
pub fn at_depth(depth: usize) -> Option<&'static CallStackElement> {
    let call_stack = read();
    if depth >= call_stack.len() {
        None
    } else {
        call_stack.get(call_stack.len() - depth - 1)
    }
}

/// Return the context of the immediate caller of the current context.
///
/// Reverts with [`ApiError::User(u16::MAX)`] if there is no immediate caller.
pub fn caller() -> &'static CallStackElement {
    at_depth(1).unwrap_or_revert_with(ApiError::User(u16::MAX))
}

/// Return the current context.
pub fn current() -> &'static CallStackElement {
    // this is infallible
    at_depth(0).unwrap_or_revert()
}

fn current_contract_full() -> (&'static ContractPackageHash, &'static ContractHash) {
    match current() {
        CallStackElement::Session { .. } => revert(ApiError::User(u16::MAX)),
        CallStackElement::StoredSession {
            contract_package_hash,
            contract_hash,
            ..
        } => (contract_package_hash, contract_hash),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash,
        } => (contract_package_hash, contract_hash),
    }
}

/// Return the current contract's package hash.
///
/// Reverts with [`ApiError::User(u16::MAX)`] if the current context doesn't reference a contract.
pub fn current_package() -> ContractPackageHash {
    *current_contract_full().0
}

/// Return the current contract's package hash.
///
/// Reverts with [`ApiError::User(u16::MAX)`] if the current context doesn't reference a contract.
pub fn current_contract() -> ContractHash {
    *current_contract_full().1
}

/// Extension trait for a call stack element.
pub trait CallStackElementEx {
    /// Derive a [`Key`] identifying the context logical 'owner' or 'invoker'.
    ///
    /// For contexts that reference an account (Session and StoredSession), the key will be the
    /// account (as [`Key::Account`]). For contexts that reference a contract (StoredContract), the
    /// key will be the contract's package hash (as [`Key::Hash`])
    fn key(&self) -> Key;

    /// Return the contract package hash if the context references a contract.
    fn contract_package(&self) -> Option<ContractPackageHash>;

    /// Return the contract hash if the context references a contract.
    fn contract(&self) -> Option<ContractHash>;
}

impl CallStackElementEx for CallStackElement {
    fn key(&self) -> Key {
        match self {
            CallStackElement::Session { account_hash } => Key::Account(*account_hash),
            CallStackElement::StoredSession { account_hash, .. } => Key::Account(*account_hash),
            CallStackElement::StoredContract {
                contract_package_hash,
                ..
            } => Key::Hash(contract_package_hash.value()),
        }
    }

    fn contract_package(&self) -> Option<ContractPackageHash> {
        match self {
            CallStackElement::Session { .. } => None,
            CallStackElement::StoredSession {
                contract_package_hash,
                ..
            } => Some(*contract_package_hash),
            CallStackElement::StoredContract {
                contract_package_hash,
                ..
            } => Some(*contract_package_hash),
        }
    }

    fn contract(&self) -> Option<ContractHash> {
        match self {
            CallStackElement::Session { .. } => None,
            CallStackElement::StoredSession { contract_hash, .. } => Some(*contract_hash),
            CallStackElement::StoredContract { contract_hash, .. } => Some(*contract_hash),
        }
    }
}
