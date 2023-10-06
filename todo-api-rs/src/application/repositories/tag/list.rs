use async_trait::async_trait;

use crate::domain::entities::tag::TagEntity;

#[async_trait]
pub trait List {
    async fn list(&self) -> Result<Vec<TagEntity>, ListError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListError {
    Internal,
}
