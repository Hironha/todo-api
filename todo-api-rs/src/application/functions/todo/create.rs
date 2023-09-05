use async_trait::async_trait;

use crate::domain::{todo::Todo, types::Date};

#[derive(Debug)]
pub enum CreateError {
    InternalError,
}

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreatePayload) -> Result<Todo, CreateError>;
}

pub struct CreateContext<T: Create> {
    pub store: T,
}

pub async fn create_todo<T: Create>(
    ctx: CreateContext<T>,
    payload: CreatePayload,
) -> Result<Todo, CreateError> {
    ctx.store.create(payload).await
}
