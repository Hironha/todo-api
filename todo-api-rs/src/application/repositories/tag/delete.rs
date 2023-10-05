use crate::domain::types::Id;
use async_trait::async_trait;

#[async_trait]
pub trait Delete {
    async fn delete(&self, id: Id) -> Result<(), DeleteError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeleteError {
    NotFound,
    Internal,
}
