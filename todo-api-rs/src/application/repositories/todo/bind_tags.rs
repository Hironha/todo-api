use async_trait::async_trait;

use crate::domain::types::Id;

#[async_trait]
pub trait BindTags {
    async fn bind_tags(&self, payload: BindTagsPayload) -> Result<(), BindTagsError>;
}

#[derive(Clone, Debug)]
pub struct BindTagsPayload {
    pub todo_id: Id,
    pub tags_id: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindTagsError {
    TodoNotFound,
    TagNotFound,
    Internal,
}
