use std::error::Error;
use std::fmt;

use crate::domain::types::{Date, DateTime, Id};

use super::tag::TagEntity;

#[derive(Clone, Debug)]
pub struct TodoEntity {
    pub id: Id,
    pub title: Title,
    pub description: Option<Description>,
    // TODO: replace `done` by a status that allows: todo, in-progress and done 
    pub done: bool,
    pub todo_at: Option<Date>,
    pub tags: Vec<TagEntity>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Title(String);

impl Title {
    pub fn new(title: impl Into<String>) -> Result<Self, TitleError> {
        let title: String = title.into();
        if title.is_empty() {
            return Err(TitleError::Empty);
        } else if title.len() > 64 {
            return Err(TitleError::Length);
        }

        Ok(Self(title))
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(description: impl Into<String>) -> Result<Self, DescriptionError> {
        let description: String = description.into();

        if description.len() > 256 {
            return Err(DescriptionError::Length);
        }

        Ok(Self(description))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleError {
    Empty,
    Length,
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "cannot be empty"),
            Self::Length => write!(f, "cannot have more than 64 characters"),
        }
    }
}

impl Error for TitleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptionError {
    Length,
}

impl fmt::Display for DescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length => write!(f, "cannot have more than 256 characters"),
        }
    }
}

impl Error for DescriptionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
