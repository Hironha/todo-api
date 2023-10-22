use async_trait::async_trait;

use crate::domain::entities::tag::{Description, Name, TagEntity};

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub name: Name,
    pub description: Description,
}

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreatePayload) -> Result<TagEntity, CreateError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateError {
    Internal,
}
