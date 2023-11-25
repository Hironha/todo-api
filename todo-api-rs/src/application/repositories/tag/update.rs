use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::Id;

#[async_trait]
pub trait Update {
    async fn update(&self, payload: UpdatePayload) -> Result<TagEntity, UpdateError>;
}

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(Debug)]
pub enum UpdateError {
    NotFound,
    Internal(Box<dyn Error>),
}

impl UpdateError {
    pub fn from_err(err: impl Error + 'static) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag not found"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for UpdateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
