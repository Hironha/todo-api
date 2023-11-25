use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::domain::entities::todo::{Description, Title, TodoStatus};
use crate::domain::types::{Date, Id};

#[async_trait]
pub trait Update {
    async fn update(&self, payload: UpdatePayload) -> Result<(), UpdateError>;
}

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: TodoStatus,
    pub todo_at: Option<Date>,
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
            Self::NotFound => write!(f, "todo not found"),
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
