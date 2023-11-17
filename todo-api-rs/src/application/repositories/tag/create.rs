use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::{DateTime, Id};

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreatePayload) -> Result<TagEntity, CreateError>;
}

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug)]
pub enum CreateError {
    Internal(Box<dyn Error>),
}

impl CreateError {
    pub fn from_err(err: impl Error + 'static) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for CreateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
