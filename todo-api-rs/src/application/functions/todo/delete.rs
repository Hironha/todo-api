use async_trait::async_trait;

use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct DeletePayload {
    pub id: String,
}

#[async_trait]
pub trait TodoDeleter {
    async fn delete(&self, id: &str) -> Result<Todo, String>;
}

pub struct DeleteContext<T: TodoDeleter> {
    pub store: T,
}

pub async fn delete_todo<T: TodoDeleter>(
    ctx: &DeleteContext<T>,
    payload: &DeletePayload,
) -> Result<Todo, String> {
    ctx.store.delete(&payload.id).await
}
