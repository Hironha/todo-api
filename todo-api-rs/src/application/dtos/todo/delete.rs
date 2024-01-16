use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeleteTodoError {
    #[error("Todo could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
