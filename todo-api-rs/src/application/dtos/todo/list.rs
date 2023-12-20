use std::error;
use std::num::NonZeroU32;

use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum ListTodoError {
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
