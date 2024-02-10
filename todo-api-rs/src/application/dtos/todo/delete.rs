use std::error;

use thiserror::Error;

use crate::domain::types::Id;

pub type DeleteTodoInput = Id;
pub type DeleteTodoOutput = Result<(), DeleteTodoError>;

#[derive(Debug, Error)]
pub enum DeleteTodoError {
    #[error("Todo could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
