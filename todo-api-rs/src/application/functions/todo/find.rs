use async_trait::async_trait;

use crate::domain::{todo::Todo, types::Id};

#[derive(Clone, Debug)]
pub struct FindPayload {
    pub id: Id,
}

#[async_trait]
pub trait Find {
    async fn find(&self, id: &Id) -> Result<Todo, String>;
}

pub struct GetContext<T: Find> {
    pub store: T,
}

pub async fn get_todo<T: Find>(ctx: GetContext<T>, payload: &FindPayload) -> Result<Todo, String> {
    ctx.store.find(&payload.id).await
}
