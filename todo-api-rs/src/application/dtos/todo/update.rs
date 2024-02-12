use std::error;

use thiserror::Error;

use crate::domain::entities::todo::{Description, Title, Status};
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug)]
pub struct UpdateTodoInput {
    pub id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: Status,
    pub todo_at: Option<Date>,
}

pub type UpdateTodoOutput = Result<(), UpdateTodoError>;

#[derive(Debug, Error)]
pub enum UpdateTodoError {
    #[error("Todo could not be found")]
    NotFound,
    #[error("Todo with title {0} already exists")]
    DuplicatedTitle(Title),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
