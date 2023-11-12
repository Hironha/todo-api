use std::error::Error;
use std::fmt;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTodoInput(pub Id);

#[derive(Debug)]
pub enum FindTodoError {
    NotFound,
    Repository(Box<dyn Error>),
}

impl fmt::Display for FindTodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "todo could not be found"),
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for FindTodoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
