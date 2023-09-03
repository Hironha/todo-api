use async_trait::async_trait;

use crate::domain::{todo::Todo, types::Id};

#[derive(Debug)]
pub enum FindError {
    NotFound,
    InternalError,
}

#[derive(Clone, Debug)]
pub struct FindPayload {
    pub id: Id,
}

#[async_trait]
pub trait Find {
    async fn find(&self, id: &Id) -> Result<Todo, FindError>;
}

pub struct FindContext<T: Find> {
    pub store: T,
}

pub async fn find_todo<T: Find>(
    ctx: FindContext<T>,
    payload: FindPayload,
) -> Result<Todo, FindError> {
    ctx.store.find(&payload.id).await
}
