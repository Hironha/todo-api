use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FindTodoError {
    #[error("todo could not be found")]
    NotFound,
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
