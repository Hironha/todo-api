use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

use async_trait::async_trait;

use crate::domain::entities::todo::{Title, TodoEntity};
use crate::domain::types::Id;

#[async_trait]
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

#[derive(Debug)]
pub enum BindTagsError {
    Internal(Box<dyn Error>),
}

impl BindTagsError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for BindTagsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for BindTagsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
        }
    }
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
            Self::NotFound => write!(f, "todo not found"),
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
pub enum ExistsError {
    Internal(Box<dyn Error>),
}

impl ExistsError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for ExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for ExistsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
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
            Self::NotFound => write!(f, "todo not found"),
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
pub enum ListError {
    Internal(Box<dyn Error>),
}

impl ListError {
    pub fn from_err(err: impl Into<Box<dyn Error>>) -> Self {
        Self::Internal(err.into())
    }
}

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl Error for ListError {
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
            Self::NotFound => write!(f, "todo not found"),
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
