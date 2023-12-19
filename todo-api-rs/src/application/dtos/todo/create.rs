use std::error;

use thiserror::Error;

use crate::domain::entities::todo::{Description, Title, TodoStatus};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: Title,
    pub description: Option<Description>,
    pub todo_at: Option<Date>,
    pub status: TodoStatus,
}

#[derive(Debug, Error)]
pub enum CreateTodoError {
    #[error("todo with title {0} already exists")]
    DuplicatedTitle(String),
    #[error(transparent)]
    Repository(#[from] Box<dyn error::Error>),
}
