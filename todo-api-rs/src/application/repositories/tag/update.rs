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
    pub description: Description,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateError {
    NotFound,
    Internal,
}
