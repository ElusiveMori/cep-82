use alloc::vec;
use alloc::vec::Vec;
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLTyped, Key, U256, U512,
};

use crate::state::{RoyaltyPaymentState, RoyaltyStep};

const ROYALTY_PAYMENT_STATE_PAID: u8 = 1;
const ROYALTY_PAYMENT_STATE_UNPAID: u8 = 0;

impl ToBytes for RoyaltyPaymentState {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        match self {
            Self::Paid {
                payer,
                source_key,
                amount,
            } => {
                let mut result = Vec::with_capacity(self.serialized_length());
                result.push(ROYALTY_PAYMENT_STATE_PAID);
                result.append(&mut payer.to_bytes()?);
                result.append(&mut source_key.to_bytes()?);
                result.append(&mut amount.to_bytes()?);
                Ok(result)
            }
            Self::Unpaid => Ok(vec![ROYALTY_PAYMENT_STATE_UNPAID]),
        }
    }

    fn serialized_length(&self) -> usize {
        match self {
            Self::Paid {
                payer,
                source_key,
                amount,
            } => {
                1 + payer.serialized_length()
                    + source_key.serialized_length()
                    + amount.serialized_length()
            }
            Self::Unpaid => 1,
        }
    }
}

impl FromBytes for RoyaltyPaymentState {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (state, bytes) = u8::from_bytes(bytes)?;
        match state {
            ROYALTY_PAYMENT_STATE_PAID => {
                let (payer, bytes) = Key::from_bytes(bytes)?;
                let (source_key, bytes) = Key::from_bytes(bytes)?;
                let (amount, bytes) = U512::from_bytes(bytes)?;
                Ok((
                    Self::Paid {
                        payer,
                        source_key,
                        amount,
                    },
                    bytes,
                ))
            }
            ROYALTY_PAYMENT_STATE_UNPAID => Ok((Self::Unpaid, bytes)),
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

const ROYALTY_STEP_MINIMUM: u8 = 0;
const ROYALTY_STEP_FLAT: u8 = 1;
const ROYALTY_STEP_PERCENTAGE: u8 = 2;

impl ToBytes for RoyaltyStep {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        match self {
            Self::Minimum { amount } => {
                let mut result = Vec::with_capacity(self.serialized_length());
                result.push(ROYALTY_STEP_MINIMUM);
                result.append(&mut amount.to_bytes()?);
                Ok(result)
            }
            Self::Flat { amount } => {
                let mut result = Vec::with_capacity(self.serialized_length());
                result.push(ROYALTY_STEP_FLAT);
                result.append(&mut amount.to_bytes()?);
                Ok(result)
            }
            Self::Percentage { percent } => {
                let mut result = Vec::with_capacity(self.serialized_length());
                result.push(ROYALTY_STEP_PERCENTAGE);
                result.append(&mut percent.to_bytes()?);
                Ok(result)
            }
        }
    }

    fn serialized_length(&self) -> usize {
        match self {
            Self::Minimum { amount } => 1 + amount.serialized_length(),
            Self::Flat { amount } => 1 + amount.serialized_length(),
            Self::Percentage { percent } => 1 + percent.serialized_length(),
        }
    }
}

impl FromBytes for RoyaltyStep {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (step, bytes) = u8::from_bytes(bytes)?;
        match step {
            ROYALTY_STEP_MINIMUM => {
                let (amount, bytes) = U512::from_bytes(bytes)?;
                Ok((Self::Minimum { amount }, bytes))
            }
            ROYALTY_STEP_FLAT => {
                let (amount, bytes) = U512::from_bytes(bytes)?;
                Ok((Self::Flat { amount }, bytes))
            }
            ROYALTY_STEP_PERCENTAGE => {
                let (percent, bytes) = U256::from_bytes(bytes)?;
                Ok((Self::Percentage { percent }, bytes))
            }
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

impl CLTyped for RoyaltyPaymentState {
    fn cl_type() -> casper_types::CLType {
        casper_types::CLType::Any
    }
}
