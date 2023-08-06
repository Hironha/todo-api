use async_trait::async_trait;

use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct GetPayload {
    pub id: String,
}

#[async_trait]
pub trait TodoGetter {
    async fn get(&self, id: &str) -> Result<Todo, String>;
}

pub struct GetContext<T: TodoGetter> {
    pub store: T,
}

pub async fn get_todo<T: TodoGetter>(
    ctx: GetContext<T>,
    payload: &GetPayload,
) -> Result<Todo, String> {
    ctx.store.get(&payload.id).await
}
