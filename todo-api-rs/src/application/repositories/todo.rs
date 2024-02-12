use std::error;
use std::num::NonZeroU32;

use thiserror::Error;

use crate::domain::entities::todo::{Description, Title, TodoEntity, Status};
use crate::domain::types::{Date, Id};

pub trait TodoRepository {
    async fn create(&mut self, todo: TodoEntity) -> Result<(), CreateError>;
    async fn delete(&mut self, todo_id: Id) -> Result<(), DeleteError>;
    async fn find(&self, todo_id: Id) -> Result<TodoEntity, FindError>;
    async fn list(&self, query: ListQuery) -> Result<PaginatedList, ListError>;
    async fn update(&mut self, query: UpdateQuery) -> Result<(), UpdateError>;
}

#[derive(Clone, Debug)]
pub struct UpdateQuery {
    pub id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: Status,
    pub todo_at: Option<Date>,
}

#[derive(Clone, Debug)]
pub struct ListQuery {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub title: Option<Title>,
}

#[derive(Clone, Debug)]
pub struct PaginatedList {
    pub count: u64,
    pub items: Vec<TodoEntity>,
}

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("Todo title already exists")]
    DuplicatedTitle,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("Todo could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum FindError {
    #[error("Todo could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum ListError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Todo could not be found")]
    NotFound,
    #[error("Todo title already exists")]
    DuplicatedTitle,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
