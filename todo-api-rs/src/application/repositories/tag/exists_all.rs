use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::types::Id;

#[async_trait]
pub trait ExistsAll {
    async fn exists_all(&self, tags_id: &[Id]) -> Result<(), ExistsAllError>;
}

#[derive(Debug)]
pub enum ExistsAllError {
    NotFound(Vec<Id>),
    Internal(Box<dyn Error>),
}

impl ExistsAllError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for ExistsAllError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(tags_id) => write!(f, "following tags were not found: {tags_id:?}"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for ExistsAllError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound(_) => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
