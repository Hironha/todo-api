use std::error;

use async_trait::async_trait;
use thiserror::Error;

use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

#[async_trait]
pub trait TagRepository {
    async fn create(&self, tag: TagEntity) -> Result<TagEntity, CreateError>;
    async fn delete(&self, tag_id: Id) -> Result<(), DeleteError>;
    async fn exists_many(&self, tag_ids: &[Id]) -> Result<(), ExistsManyError>;
    async fn find(&self, tag_id: Id) -> Result<TagEntity, FindError>;
    async fn list_all(&self) -> Result<Vec<TagEntity>, ListAllError>;
    async fn update(&self, tag: TagEntity) -> Result<TagEntity, UpdateError>;
}

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("tag name already exists")]
    DuplicatedName,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("tag could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum ExistsManyError {
    #[error("following tags were not found: {0:?}")]
    NotFound(Vec<Id>),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum FindError {
    #[error("tag could not be found")]
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
    #[error("tag could not be found")]
    NotFound,
    #[error("tag name already exists")]
    DuplicatedName,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
