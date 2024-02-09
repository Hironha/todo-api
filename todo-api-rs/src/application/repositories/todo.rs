use std::error;
use std::num::NonZeroU32;

use thiserror::Error;

use crate::domain::entities::todo::{Title, TodoEntity};
use crate::domain::types::Id;

pub trait TodoRepository {
    async fn bind_tags(&mut self, todo_id: Id, tag_ids: &[Id]) -> Result<(), BindTagsError>;
    async fn create(&mut self, todo: TodoEntity) -> Result<TodoEntity, CreateError>;
    async fn delete(&mut self, todo_id: Id) -> Result<(), DeleteError>;
    async fn exists(&mut self, todo_id: Id) -> Result<bool, ExistsError>;
    async fn find(&mut self, todo_id: Id) -> Result<TodoEntity, FindError>;
    async fn list(&mut self, query: ListQuery) -> Result<PaginatedList, ListError>;
    async fn update(&mut self, todo: TodoEntity) -> Result<(), UpdateError>;
    async fn exists_tags(&mut self, tag_ids: &[Id]) -> Result<(), ExistsTagsError>;
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
pub enum BindTagsError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
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
pub enum ExistsError {
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

#[derive(Debug, Error)]
pub enum ExistsTagsError {
    #[error("Following tags were not found: {0:?}")]
    NotFound(Vec<Id>),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
