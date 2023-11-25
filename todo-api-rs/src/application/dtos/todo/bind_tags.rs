use std::error::Error;
use std::fmt;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct BindTodoTagsInput {
    pub todo_id: Id,
    pub tags_id: Vec<Id>,
}

#[derive(Debug)]
pub enum BindTodoTagsError {
    TodoNotFound,
    TagNotFound,
    Repository(Box<dyn Error>),
}

impl fmt::Display for BindTodoTagsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TagNotFound => write!(f, "tag could not be found"),
            Self::TodoNotFound => write!(f, "todo could not be found"),
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for BindTodoTagsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::TodoNotFound | Self::TagNotFound => None,
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
