use async_trait::async_trait;

use crate::domain::entities::tag::{Name, Description, TagEntity};
use crate::domain::types::{Id, DateTime};

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreatePayload) -> Result<TagEntity, CreateError>;
}

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub id: Id,
    pub name: Name,
    pub description: Description,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateError {
    Internal,
}
