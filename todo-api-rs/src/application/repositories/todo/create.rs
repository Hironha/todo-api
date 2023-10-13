use async_trait::async_trait;

use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::domain::types::Date;

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreatePayload) -> Result<TodoEntity, CreateError>;
}

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
    pub done: bool
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateError {
    Internal,
}
