use std::error::Error;
use std::fmt;

use crate::domain::entities::todo::{Description, Title};
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug)]
pub struct UpdateTodoInput {
    pub id: Id,
    pub title: Title,
    pub description: Description,
    pub done: bool,
    pub todo_at: Option<Date>,
}

#[derive(Debug)]
pub enum UpdateTodoError {
    NotFound,
    Repository(Box<dyn Error>),
}

impl fmt::Display for UpdateTodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "todo could not be found"),
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for UpdateTodoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
