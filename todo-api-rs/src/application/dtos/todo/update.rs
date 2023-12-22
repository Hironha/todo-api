use std::error;

use thiserror::Error;

use crate::domain::entities::todo::{Description, Title, TodoStatus};
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug)]
pub struct UpdateTodoInput {
    pub id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: TodoStatus,
    pub todo_at: Option<Date>,
}

#[derive(Debug, Error)]
pub enum UpdateTodoError {
    #[error("todo could not be found")]
    NotFound,
    #[error("todo with title {0} already exists")]
    DuplicatedTitle(String),
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
