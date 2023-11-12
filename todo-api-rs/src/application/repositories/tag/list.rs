use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::entities::tag::TagEntity;

#[async_trait]
pub trait List {
    async fn list(&self) -> Result<Vec<TagEntity>, ListError>;
}

#[derive(Debug)]
pub enum ListError {
    Internal(Box<dyn Error>),
}

impl ListError {
    pub fn from_err(err: impl Error + 'static) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for ListError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
