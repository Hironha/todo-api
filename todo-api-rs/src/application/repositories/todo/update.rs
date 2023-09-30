use async_trait::async_trait;

use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::domain::types::{Date, Id};

#[async_trait]
pub trait Update {
    async fn set(&self, payload: UpdatePayload) -> Result<TodoEntity, UpdateError>;
}

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: Id,
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateError {
    NotFound,
    Internal,
}
