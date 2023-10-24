use async_trait::async_trait;

use crate::domain::entities::tag::TagEntity;

#[async_trait]
pub trait Create {
    async fn create(&self, entity: TagEntity) -> Result<TagEntity, CreateError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateError {
    Internal,
}
