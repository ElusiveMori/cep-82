use contract_common::{prelude::*, FromNamedArg};
use num_traits::AsPrimitive;

use crate::CustodialError;

named_keys! {
    init_all(manager: Key, royalty_structure: RoyaltyStructure):
    dict whitelisted_marketplaces: bool;
    dict royalty_payments: RoyaltyPaymentState;

    val marketplace_whitelist_enabled: bool = false;
    val manager: Key = manager;
    val royalty_structure: RoyaltyStructure = royalty_structure;
}

pub fn is_marketplace_whitelisted(marketplace: ContractPackageHash) -> bool {
    whitelisted_marketplaces::try_read(&b64_cl(&marketplace)).unwrap_or(false)
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoyaltyPaymentState {
    Unpaid,
    Paid {
        payer: Key,
        source_key: Key,
        amount: U512,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoyaltyStep {
    Minimum { amount: U512 },
    Flat { amount: U512 },
    Percentage { percent: U256 },
}

serializable_structs! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RoyaltyStructure {
        pub steps: Vec<RoyaltyStep>,
    }
}

impl RoyaltyStructure {
    pub fn calculate_total_royalty(&self, total_payment: U512) -> U512 {
        let mut payment = total_payment;
        let mut total_royalty = U512::zero();
        for step in &self.steps {
            match step {
                RoyaltyStep::Minimum { amount } => {
                    if payment < *amount {
                        payment = *amount;
                    }
                }
                RoyaltyStep::Flat { amount } => {
                    total_royalty = total_royalty
                        .checked_add(*amount)
                        .unwrap_or_revert_with(CustodialError::Overflow);
                }
                RoyaltyStep::Percentage { percent } => {
                    total_royalty = total_royalty
                        .checked_add(
                            total_payment
                                .checked_mul(percent.as_())
                                .unwrap_or_revert_with(CustodialError::Overflow)
                                .checked_div(10000u64.into())
                                .unwrap_or_revert_with(CustodialError::Overflow),
                        )
                        .unwrap_or_revert_with(CustodialError::Overflow);
                }
            }
        }

        total_royalty
    }
}

impl FromNamedArg for RoyaltyStructure {}
