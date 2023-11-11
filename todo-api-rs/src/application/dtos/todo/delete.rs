use std::error::Error;
use std::fmt;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTodoInput(pub Id);

#[derive(Debug)]
pub enum DeleteTodoError {
    NotFound,
    Repository(Box<dyn Error>),
}

impl fmt::Display for DeleteTodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "todo could not be found"),
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for DeleteTodoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
