use std::fmt;

use thiserror::Error;

use crate::domain::types::{DateTime, Id};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagEntity {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name(String);

impl Name {
    pub const MAX_LENGTH: usize = 64;

    pub fn new(name: impl Into<String>) -> Result<Self, NameError> {
        let name: String = name.into();
        if name.is_empty() {
            return Err(NameError::Empty);
        } else if name.len() > Self::MAX_LENGTH {
            return Err(NameError::Length);
        }

        Ok(Self(name))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Description(String);

impl Description {
    pub const MAX_LENGTH: usize = 128;

    pub fn new(description: impl Into<String>) -> Result<Self, DescriptionError> {
        let description: String = description.into();
        if description.len() > Self::MAX_LENGTH {
            return Err(DescriptionError::Length);
        }

        Ok(Self(description))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum NameError {
    #[error("Tag name cannot be empty")]
    Empty,
    #[error("Tag name cannot have more than 64 characters")]
    Length,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DescriptionError {
    #[error("Tag description cannot have more than 64 characters")]
    Length,
}
