use std::error::Error;
use std::fmt;

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

// TODO: validate minimum length to `Name`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NameError {
    Empty,
    Length,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "cannot be empty"),
            Self::Length => write!(f, "cannot have more than 64 characters"),
        }
    }
}

impl Error for NameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Empty | Self::Length => None,
        }
    }
}

// TODO: validate minimum length to `Description`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptionError {
    Length,
}

impl fmt::Display for DescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length => write!(f, "cannot have more than 128 characters"),
        }
    }
}

impl Error for DescriptionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Length => None,
        }
    }
}
