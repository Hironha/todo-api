use std::error;

use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct BindTodoTagsInput {
    pub todo_id: Id,
    pub tags_id: Vec<Id>,
}

#[derive(Debug, Error)]
pub enum BindTodoTagsError {
    #[error("todo could not be found")]
    TodoNotFound,
    #[error("following tags could not be found: {0:?}")]
    TagNotFound(Vec<Id>),
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
