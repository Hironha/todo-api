use std::error::Error;
use std::fmt;

use crate::domain::types::{DateTime, Id};

#[derive(Clone, Debug)]
pub struct TagEntity {
    pub id: Id,
    pub name: Name,
    pub description: Description,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug)]
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

    pub fn new_unchecked(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

// TODO: remove `Option` from description, it's better to have
// `Option<Description>` in `TagEntity`
#[derive(Clone, Debug)]
pub struct Description(Option<String>);

impl Description {
    pub fn new(description: Option<impl Into<String>>) -> Result<Self, DescriptionError> {
        let description: Option<String> = description.map(|v| v.into()).filter(|v| !v.is_empty());
        let Some(description) = description else {
            return Ok(Self(None));
        };

        if description.len() > 128 {
            return Err(DescriptionError::Length);
        }

        Ok(Self(Some(description)))
    }

    pub fn new_unchecked(description: Option<impl Into<String>>) -> Self {
        Self(description.map(|d| d.into()))
    }

    pub fn into_opt_string(self) -> Option<String> {
        self.0
    }
}

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
