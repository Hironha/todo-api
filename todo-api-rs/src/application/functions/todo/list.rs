use async_trait::async_trait;

use crate::domain::todo::Todo;

pub enum ListError {
    StorageAccess,
}

#[async_trait]
pub trait List {
    async fn list(&self) -> Result<Vec<Todo>, ListError>;
}

pub struct ListContext<T: List> {
    pub store: T,
}

pub async fn list_todo<T: List>(ctx: ListContext<T>) -> Result<Vec<Todo>, ListError> {
    ctx.store.list().await
}
