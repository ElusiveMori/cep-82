use casper_types::{CLType, CLTyped, NamedArg};

use crate::{prelude::*, FromNamedArg};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenIdentifier {
    Index(u64),
    Hash(String),
}

impl TokenIdentifier {
    pub fn try_load_from_runtime_args() -> Option<Self> {
        if let Ok(token_id) = try_get_named_arg::<u64>("token_id") {
            Some(TokenIdentifier::Index(token_id))
        } else if let Ok(token_hash) = try_get_named_arg::<String>("token_hash") {
            Some(TokenIdentifier::Hash(token_hash))
        } else {
            None
        }
    }

    pub fn to_named_arg(&self) -> NamedArg {
        match self {
            TokenIdentifier::Index(index) => {
                NamedArg::new("token_id".into(), CLValue::from_t(*index).unwrap())
            }
            TokenIdentifier::Hash(hash) => {
                NamedArg::new("token_hash".into(), CLValue::from_t(hash.clone()).unwrap())
            }
        }
    }
}

impl ToBytes for TokenIdentifier {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        match self {
            TokenIdentifier::Index(index) => {
                let mut result = bytesrepr::allocate_buffer(self)?;
                result.push(0);
                result.append(&mut index.to_bytes()?);
                Ok(result)
            }
            TokenIdentifier::Hash(hash) => {
                let mut result = bytesrepr::allocate_buffer(self)?;
                result.push(1);
                result.append(&mut hash.to_bytes()?);
                Ok(result)
            }
        }
    }

    fn serialized_length(&self) -> usize {
        match self {
            TokenIdentifier::Index(index) => 1 + index.serialized_length(),
            TokenIdentifier::Hash(hash) => 1 + hash.serialized_length(),
        }
    }
}

impl FromBytes for TokenIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (tag, rem) = u8::from_bytes(bytes)?;
        match tag {
            0 => {
                let (index, rem) = u64::from_bytes(rem)?;
                Ok((TokenIdentifier::Index(index), rem))
            }
            1 => {
                let (hash, rem) = String::from_bytes(rem)?;
                Ok((TokenIdentifier::Hash(hash), rem))
            }
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

impl CLTyped for TokenIdentifier {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl FromNamedArg for TokenIdentifier {
    fn try_get(_: &str) -> Option<Self> {
        TokenIdentifier::try_load_from_runtime_args()
    }
}
