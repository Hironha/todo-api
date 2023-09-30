use async_trait::async_trait;

use crate::domain::entities::todo::TodoEntity;

#[async_trait]
pub trait List {
    async fn list(&self) -> Result<Vec<TodoEntity>, ListError>;
}

pub enum ListError {
    Internal,
}
