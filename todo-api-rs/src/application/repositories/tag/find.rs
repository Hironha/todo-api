use async_trait::async_trait;

use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

#[async_trait]
pub trait Find {
    async fn find(&self, id: Id) -> Result<TagEntity, FindError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindError {
    NotFound,
    Internal,
}
