use async_trait::async_trait;

use crate::domain::types::Id;

#[async_trait]
pub trait Delete {
    async fn delete(&self, id: Id) -> Result<(), DeleteError>;
}

#[derive(Debug, PartialEq)]
pub enum DeleteError {
    NotFound,
    Internal,
}
