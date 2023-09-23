use async_trait::async_trait;

use crate::domain::todo::{Description, Title, Todo};
use crate::domain::types::{Date, Id};

#[async_trait]
pub trait Update {
    async fn set(&self, payload: UpdatePayload) -> Result<Todo, UpdateError>;
}

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: Id,
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

#[derive(Debug, PartialEq)]
pub enum UpdateError {
    NotFound,
    Internal,
}
