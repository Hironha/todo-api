use std::error;

use thiserror::Error;

use crate::domain::entities::todo::{Description, Title, TodoEntity, Status};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: Title,
    pub description: Option<Description>,
    pub todo_at: Option<Date>,
    pub status: Status,
}

pub type CreateTodoOutput = Result<TodoEntity, CreateTodoError>;

#[derive(Debug, Error)]
pub enum CreateTodoError {
    #[error("Todo with title {0} already exists")]
    DuplicatedTitle(Title),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
