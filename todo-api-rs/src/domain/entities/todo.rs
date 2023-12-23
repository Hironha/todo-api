use std::fmt;

use thiserror::Error;

use crate::domain::types::{Date, DateTime, Id};

use super::tag::TagEntity;

#[derive(Clone, Debug)]
pub struct TodoEntity {
    pub id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: TodoStatus,
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

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TodoStatus {
    Todo,
    InProgress,
    Done,
}

impl TodoStatus {
    pub fn into_string(self) -> String {
        match self {
            Self::Todo => String::from("todo"),
            Self::InProgress => String::from("in_progress"),
            Self::Done => String::from("done"),
        }
    }

    pub fn parse_str(value: &str) -> Result<Self, ParseTodoStatusError> {
        match value {
            "todo" => Ok(Self::Todo),
            "in_progress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(ParseTodoStatusError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TitleError {
    #[error("todo title cannot be empty")]
    Empty,
    #[error("todo title cannot have more than 64 characters")]
    Length,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DescriptionError {
    #[error("todo description cannot have more than 64 characters")]
    Length,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error("todo status must be one the following values: todo, in_progress or done")]
pub struct ParseTodoStatusError;
