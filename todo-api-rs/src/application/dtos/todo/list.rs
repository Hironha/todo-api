use std::error;
use std::num::NonZeroU32;

use thiserror::Error;

use crate::domain::entities::todo::{Title, TodoEntity};

#[derive(Clone, Debug)]
pub struct ListTodosInput {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub title: Option<Title>,
}

pub type ListTodosOutput = Result<TodosList, ListTodosError>;

#[derive(Clone, Debug)]
pub struct TodosList {
    pub count: u64,
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub items: Vec<TodoEntity>,
}

#[derive(Debug, Error)]
pub enum ListTodosError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
