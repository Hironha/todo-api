use crate::domain::types::{Date, DateTime, Id};

#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: Id,
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq)]
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

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Description(Option<String>);
impl Description {
    pub fn new(description: Option<impl Into<String>>) -> Result<Self, DescriptionError> {
        let description: Option<String> = description.map(|d| d.into()).filter(|d| !d.is_empty());
        let Some(d) = description else {
            return Ok(Self(None));
        };
        if d.len() > 256 {
            return Err(DescriptionError::Length);
        }

        Ok(Self(Some(d)))
    }

    pub fn value(self) -> Option<String> {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleError {
    Empty,
    Length,
}
impl TitleError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Empty => "cannot be empty",
            Self::Length => "cannot have more than 64 characters",
        };
        description.into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptionError {
    Length,
}
impl DescriptionError {
    pub fn description(&self) -> String {
        match self {
            Self::Length => "cannot have more than 256 characters".into(),
        }
    }
}
