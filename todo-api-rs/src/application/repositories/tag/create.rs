use async_trait::async_trait;

use crate::domain::entities::tag::{Description, Name, TagEntity};

#[derive(Clone, Debug)]
pub struct CreateTagPayload {
    pub name: Name,
    pub description: Description,
}

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreateTagPayload) -> Result<TagEntity, CreateTagError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTagError {
    Internal,
}
