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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindTagsError {
    TodoNotFound,
    TagNotFound,
    Internal,
}
