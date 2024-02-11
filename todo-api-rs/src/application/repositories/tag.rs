use std::error;

use thiserror::Error;

use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::Id;

pub trait TagRepository {
    async fn create(&mut self, tag: TagEntity) -> Result<TagEntity, CreateError>;
    async fn delete(&mut self, tag_id: Id) -> Result<(), DeleteError>;
    async fn exists_many(&self, tag_ids: &[Id]) -> Result<(), ExistsManyError>;
    async fn find(&self, tag_id: Id) -> Result<TagEntity, FindError>;
    async fn list_all(&self) -> Result<Vec<TagEntity>, ListAllError>;
    async fn update(&mut self, query: UpdateQuery) -> Result<(), UpdateError>;
}

#[derive(Clone, Debug)]
pub struct UpdateQuery {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("Tag name already exists")]
    DuplicatedName,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("Tag could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum ExistsManyError {
    #[error("Following tags were not found: {0:?}")]
    NotFound(Vec<Id>),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum FindError {
    #[error("Tag could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum ListAllError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Tag could not be found")]
    NotFound,
    #[error("Tag name already exists")]
    DuplicatedName,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
