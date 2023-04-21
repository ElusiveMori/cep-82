use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CommonError {
    InvalidTokenIdentifier,
    InvalidMethodAccess,

    InvalidCacheSet,
}

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> Self {
        ApiError::User(error as u16)
    }
}

impl From<CommonError> for u16 {
    fn from(error: CommonError) -> Self {
        error as u16
    }
}
