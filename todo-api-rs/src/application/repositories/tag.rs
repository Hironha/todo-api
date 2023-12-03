use std::error::Error;
use std::fmt;

use async_trait::async_trait;

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

#[derive(Debug)]
pub enum CreateError {
    Internal(Box<dyn Error>),
}

impl CreateError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for CreateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug)]
pub enum DeleteError {
    NotFound,
    Internal(Box<dyn Error>),
}

impl DeleteError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for DeleteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag not found"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for DeleteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug)]
pub enum ExistsManyError {
    NotFound(Vec<Id>),
    Internal(Box<dyn Error>),
}

impl ExistsManyError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for ExistsManyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(tags_id) => write!(f, "following tags were not found: {tags_id:?}"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for ExistsManyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound(_) => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug)]
pub enum FindError {
    NotFound,
    Internal(Box<dyn Error>),
}

impl FindError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for FindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag not found"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for FindError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug)]
pub enum ListAllError {
    Internal(Box<dyn Error>),
}

impl ListAllError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for ListAllError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for ListAllError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug)]
pub enum UpdateError {
    NotFound,
    Internal(Box<dyn Error>),
}

impl UpdateError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag not found"),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for UpdateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
}
