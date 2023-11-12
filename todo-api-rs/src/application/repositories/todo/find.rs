use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::Id;

#[async_trait]
pub trait Find {
    async fn find(&self, id: Id) -> Result<TodoEntity, FindError>;
}

#[derive(Debug)]
pub enum FindError {
    NotFound,
    Internal(Box<dyn Error>),
}

impl FindError {
    pub fn from_err(err: impl Error + 'static) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for FindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "todo not found"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for FindError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
