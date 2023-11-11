use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

use async_trait::async_trait;

use crate::domain::entities::todo::{Title, TodoEntity};

#[async_trait]
pub trait List {
    async fn list(&self, payload: ListPayload) -> Result<ListData, ListError>;
}

#[derive(Clone, Debug)]
pub struct ListPayload {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub title: Option<Title>,
}

#[derive(Clone, Debug)]
pub struct ListData {
    pub count: u64,
    pub items: Vec<TodoEntity>,
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
