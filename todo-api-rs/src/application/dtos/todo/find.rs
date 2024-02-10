use std::error;

use thiserror::Error;

use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::Id;

pub type FindTodoInput = Id;
pub type FindTodoOutput = Result<TodoEntity, FindTodoError>;

#[derive(Debug, Error)]
pub enum FindTodoError {
    #[error("Todo could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
