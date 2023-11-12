use std::error::Error;

use async_trait::async_trait;

use crate::domain::types::{DateTime, Id};

#[async_trait]
pub trait BindTags {
    async fn bind_tags(&self, payload: BindTagsPayload) -> Result<(), BindTagsError>;
}

#[derive(Clone, Debug)]
pub struct BindTagsPayload {
    pub todo_id: Id,
    pub tags_id: Option<Vec<Id>>,
    pub current_dt: DateTime,
}

#[derive(Debug)]
pub enum BindTagsError {
    TodoNotFound,
    TagNotFound,
    Internal(Box<dyn Error>),
}

impl BindTagsError {
    pub fn from_err(err: impl Error + 'static) -> Self {
        Self::Internal(err.into())
    }
}
