use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::types::Id;

#[async_trait]
pub trait Delete {
    async fn delete(&self, id: Id) -> Result<(), DeleteError>;
}

#[derive(Debug)]
pub enum DeleteError {
    NotFound,
    Internal(Box<dyn Error>),
}

impl DeleteError {
    pub fn from_err(err: impl Error + 'static) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for DeleteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag not found"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for DeleteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
