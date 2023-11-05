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
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let name: String = value.into();
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

#[derive(Clone, Debug)]
pub struct Description(Option<String>);

impl Description {
    pub fn new(value: Option<impl Into<String>>) -> Result<Self, DescriptionError> {
        let description: Option<String> = value.map(|v| v.into()).filter(|v| !v.is_empty());
        let Some(description) = description else {
            return Ok(Self(None));
        };

        if description.len() > 128 {
            return Err(DescriptionError::Length);
        }

        Ok(Self(Some(description)))
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

impl std::fmt::Display for NameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "cannot be empty"),
            Self::Length => write!(f, "cannot have more than 64 characters"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptionError {
    Length,
}

impl std::fmt::Display for DescriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Length => write!(f, "cannot have more than 128 characters"),
        }
    }
}
