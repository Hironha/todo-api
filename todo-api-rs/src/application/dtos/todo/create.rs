use std::error::Error;
use std::fmt;

use crate::application::repositories::todo::CreateError;
use crate::domain::entities::todo::{Description, Title, TodoStatus};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: Title,
    pub description: Option<Description>,
    pub todo_at: Option<Date>,
    pub status: TodoStatus,
}

#[derive(Debug)]
pub enum CreateTodoError {
    Creating(CreateError),
}

impl fmt::Display for CreateTodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creating(err) => err.fmt(f),
        }
    }
}

impl Error for CreateTodoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Creating(err) => Some(err),
        }
    }
}
