use async_trait::async_trait;

use crate::domain::entities::todo::Todo;
use crate::domain::types::Id;

#[async_trait]
pub trait Find {
    async fn find(&self, id: Id) -> Result<Todo, FindError>;
}

#[derive(Debug, PartialEq)]
pub enum FindError {
    NotFound,
    Internal,
}
