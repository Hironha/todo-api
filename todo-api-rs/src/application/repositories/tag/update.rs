use async_trait::async_trait;

use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::{DateTime, Id};

#[async_trait]
pub trait Update {
    async fn update(&self, payload: UpdatePayload) -> Result<TagEntity, UpdateError>;
}

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: Id,
    pub name: Name,
    pub description: Description,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateError {
    NotFound,
    Internal,
}
