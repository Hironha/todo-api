use async_trait::async_trait;

use crate::domain::{
    todo::Todo,
    types::{Date, Id},
};

#[derive(Clone, Debug)]
pub enum UpdateError {
    NotFound,
    InternalError,
}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "NotFound"),
            Self::InternalError => write!(f, "InternalError"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

#[async_trait]
pub trait Update {
    async fn set(&self, payload: UpdatePayload) -> Result<Todo, UpdateError>;
}

pub struct UpdateContext<T: Update> {
    pub store: T,
}

pub async fn update_todo<T: Update>(
    ctx: &UpdateContext<T>,
    payload: UpdatePayload,
) -> Result<Todo, UpdateError> {
    ctx.store.set(payload).await
}
