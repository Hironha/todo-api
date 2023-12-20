use std::error;

use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTodoInput(pub Id);

#[derive(Debug, Error)]
pub enum DeleteTodoError {
    #[error("todo could not be found")]
    NotFound,
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
