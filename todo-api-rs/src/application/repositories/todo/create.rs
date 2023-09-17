use async_trait::async_trait;

use crate::domain::todo::{Description, Title, Todo};
use crate::domain::types::Date;

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreatePayload) -> Result<Todo, CreateError>;
}

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateError {
    Internal,
}
