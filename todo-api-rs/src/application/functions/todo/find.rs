use async_trait::async_trait;

use crate::domain::{todo::Todo, types::Id};

#[derive(Clone, Debug)]
pub enum FindError {
    NotFound,
    InternalError,
}

impl std::fmt::Display for FindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "NotFound"),
            Self::InternalError => write!(f, "InternalError"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FindPayload {
    pub id: Id,
}

#[async_trait]
pub trait Find {
    async fn find(&self, id: &Id) -> Result<Todo, FindError>;
}

pub struct GetContext<T: Find> {
    pub store: T,
}

pub async fn find_todo<T: Find>(
    ctx: GetContext<T>,
    payload: &FindPayload,
) -> Result<Todo, FindError> {
    ctx.store.find(&payload.id).await
}
