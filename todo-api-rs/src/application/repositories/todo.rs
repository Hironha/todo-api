use std::error;
use std::num::NonZeroU32;

use thiserror::Error;

use crate::domain::entities::todo::{Title, TodoEntity};
use crate::domain::types::Id;

pub trait TodoRepository {
    async fn bind_tags(&self, todo_id: Id, tag_ids: Vec<Id>) -> Result<(), BindTagsError>;
    async fn create(&self, todo: TodoEntity) -> Result<TodoEntity, CreateError>;
    async fn delete(&self, todo_id: Id) -> Result<(), DeleteError>;
    async fn exists(&self, todo_id: Id) -> Result<bool, ExistsError>;
    async fn find(&self, todo_id: Id) -> Result<TodoEntity, FindError>;
    async fn list(&self, payload: ListPayload) -> Result<PaginatedList, ListError>;
    async fn update(&self, todo: TodoEntity) -> Result<(), UpdateError>;
}

#[derive(Clone, Debug)]
pub struct ListPayload {
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
pub enum BindTagsError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("todo title already exists")]
    DuplicatedTitle,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("todo could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum ExistsError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum FindError {
    #[error("todo could not be found")]
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
    #[error("todo could not be found")]
    NotFound,
    #[error("todo title already exists")]
    DuplicatedTitle,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
