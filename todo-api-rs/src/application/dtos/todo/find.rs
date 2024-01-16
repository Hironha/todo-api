use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FindTodoError {
    #[error("Todo could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
