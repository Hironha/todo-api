use std::error;

use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct BindTodoTagsInput {
    pub todo_id: Id,
    pub tags_id: Vec<Id>,
}

pub type BindTodoTagsOutput = Result<(), BindTodoTagsError>;

#[derive(Debug, Error)]
pub enum BindTodoTagsError {
    #[error("Todo could not be found")]
    TodoNotFound,
    #[error("Following tags could not be found: {0:?}")]
    TagNotFound(Vec<Id>),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
