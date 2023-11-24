use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::types::Id;

#[async_trait]
pub trait Exists {
    async fn exists(&self, todo_id: Id) -> Result<bool, ExistsError>;
}

#[derive(Debug)]
pub enum ExistsError {
    Internal(Box<dyn Error>),
}

impl ExistsError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for ExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for ExistsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
