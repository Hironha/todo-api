use async_trait::async_trait;

use crate::domain::todo::Todo;

#[async_trait]
pub trait TodoLister {
    async fn list(&self) -> Result<Vec<Todo>, String>;
}

pub struct ListContext<T: TodoLister> {
    pub store: T,
}

pub async fn list_todo<T: TodoLister>(ctx: &ListContext<T>) -> Result<Vec<Todo>, String> {
    ctx.store.list().await
}
