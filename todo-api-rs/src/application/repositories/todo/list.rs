use async_trait::async_trait;

use crate::domain::entities::todo::Todo;

#[async_trait]
pub trait List {
    async fn list(&self) -> Result<Vec<Todo>, ListError>;
}

pub enum ListError {
    Internal,
}
