use thiserror::Error;

use crate::domain::types::{DateTime, Id};

#[derive(Clone, Debug)]
pub struct TagEntity {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new(name: impl Into<String>) -> Result<Self, NameError> {
        let name: String = name.into();
        if name.is_empty() {
            return Err(NameError::Empty);
        } else if name.len() > 64 {
            return Err(NameError::Length);
        }

        Ok(Self(name))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(description: impl Into<String>) -> Result<Self, DescriptionError> {
        let description: String = description.into();

        if description.len() > 128 {
            return Err(DescriptionError::Length);
        }

        Ok(Self(description))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum NameError {
    #[error("tag name cannot be empty")]
    Empty,
    #[error("tag name cannot have more than 64 characters")]
    Length,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DescriptionError {
    #[error("tag description cannot have more than 64 characters")]
    Length,
}
