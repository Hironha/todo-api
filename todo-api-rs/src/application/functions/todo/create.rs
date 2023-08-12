use async_trait::async_trait;
use time::Date;

use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

#[async_trait]
pub trait TodoCreator {
    async fn create(&self, payload: CreatePayload) -> Result<Todo, String>;
}

pub struct CreateContext<T: TodoCreator> {
    pub store: T,
}

pub async fn create_todo<T: TodoCreator>(
    ctx: &CreateContext<T>,
    payload: CreatePayload,
) -> Result<Todo, String> {
    ctx.store.create(payload).await
}
