use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct GetPayload {
    pub id: Uuid,
}

#[async_trait]
pub trait Find {
    async fn find(&self, id: &Uuid) -> Result<Todo, String>;
}

pub struct GetContext<T: Find> {
    pub store: T,
}

pub async fn get_todo<T: Find>(
    ctx: GetContext<T>,
    payload: &GetPayload,
) -> Result<Todo, String> {
    ctx.store.find(&payload.id).await
}
