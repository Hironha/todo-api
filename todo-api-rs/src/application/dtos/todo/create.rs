use std::error::Error;
use std::fmt;

use crate::domain::entities::todo::{Description, Title};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: Title,
    pub description: Option<Description>,
    pub todo_at: Option<Date>,
}

#[derive(Debug)]
pub enum CreateTodoError {
    Repository(Box<dyn Error>),
}

impl fmt::Display for CreateTodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for CreateTodoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
