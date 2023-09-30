use async_trait::async_trait;

use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::Id;

#[async_trait]
pub trait Find {
    async fn find(&self, id: Id) -> Result<TodoEntity, FindError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindError {
    NotFound,
    Internal,
}
