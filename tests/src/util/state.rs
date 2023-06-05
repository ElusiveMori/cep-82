use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLTyped, U256, U512,
};

const ROYALTY_STEP_MINIMUM: u8 = 0;
const ROYALTY_STEP_FLAT: u8 = 1;
const ROYALTY_STEP_PERCENTAGE: u8 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoyaltyStep {
    Minimum { amount: U512 },
    Flat { amount: U512 },
    Percentage { percent: U256 },
}

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

impl CLTyped for RoyaltyStep {
    fn cl_type() -> casper_types::CLType {
        casper_types::CLType::Any
    }
}

impl RoyaltyStep {
    pub fn basic() -> Vec<Self> {
        vec![Self::Flat {
            amount: U512::from(100),
        }]
    }
}
