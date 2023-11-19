use std::error::Error;

use async_trait::async_trait;

use crate::domain::entities::todo::{Description, Title, TodoEntity, TodoEntityStatus};
use crate::domain::types::{Date, DateTime};

#[async_trait]
pub trait Create {
    async fn create(&self, payload: CreatePayload) -> Result<TodoEntity, CreateError>;
}

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub title: Title,
    pub description: Option<Description>,
    pub todo_at: Option<Date>,
    pub status: TodoEntityStatus,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug)]
pub enum CreateError {
    Internal(Box<dyn Error>),
}

impl CreateError {
    pub fn from_err(err: impl Error + 'static) -> Self {
        Self::Internal(err.into())
    }
}
