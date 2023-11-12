use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

use crate::domain::entities::todo::{Title, TodoEntity};

#[derive(Clone, Debug)]
pub struct ListTodoInput {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub title: Option<Title>,
}

#[derive(Clone, Debug)]
pub struct TodoList {
    pub count: u64,
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub items: Vec<TodoEntity>,
}

#[derive(Debug)]
pub enum ListTodoError {
    Repository(Box<dyn Error>),
}

impl fmt::Display for ListTodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for ListTodoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
