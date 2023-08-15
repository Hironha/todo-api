use async_trait::async_trait;

use crate::domain::todo::Todo;

#[async_trait]
pub trait List {
    async fn list(&self) -> Result<Vec<Todo>, String>;
}

pub struct ListContext<T: List> {
    pub store: T,
}

pub async fn list_todo<T: List>(ctx: &ListContext<T>) -> Result<Vec<Todo>, String> {
    ctx.store.list().await
}
