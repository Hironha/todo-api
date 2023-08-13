use async_trait::async_trait;
use time::Date;
use uuid::Uuid;

use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

#[async_trait]
pub trait TodoSetter {
    async fn set(&self, payload: UpdatePayload) -> Result<Todo, String>;
}

pub struct UpdateContext<T: TodoSetter> {
    pub store: T,
}

pub async fn update_todo<T: TodoSetter>(
    ctx: &UpdateContext<T>,
    payload: UpdatePayload,
) -> Result<Todo, String> {
    ctx.store.set(payload).await
}
