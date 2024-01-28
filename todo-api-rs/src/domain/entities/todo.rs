use std::fmt;

use thiserror::Error;

use crate::domain::types::{Date, DateTime, Id};

use super::tag::TagEntity;

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Title(String);

impl Title {
    pub const MAX_LENGTH: usize = 64;

    pub fn new(title: impl Into<String>) -> Result<Self, TitleError> {
        let title: String = title.into();
        if title.is_empty() {
            return Err(TitleError::Empty);
        } else if title.len() > Self::MAX_LENGTH {
            return Err(TitleError::Length);
        }

        Ok(Self(title))
    }

    pub fn into_inner(self) -> String {
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Description(String);

impl Description {
    pub const MAX_LENGTH: usize = 256;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TodoStatus {
    Todo,
    InProgress,
    Done,
}

impl TodoStatus {
    const TODO_STR: &'static str = "todo";
    const IN_PROGRESS_STR: &'static str = "in_progress";
    const DONE_STR: &'static str = "done";

    pub fn parse_str(value: &str) -> Result<Self, ParseTodoStatusError> {
        match value {
            Self::TODO_STR => Ok(Self::Todo),
            Self::IN_PROGRESS_STR => Ok(Self::InProgress),
            Self::DONE_STR => Ok(Self::Done),
            _ => Err(ParseTodoStatusError),
        }
    }
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Todo => f.write_str(Self::TODO_STR),
            Self::InProgress => f.write_str(Self::IN_PROGRESS_STR),
            Self::Done => f.write_str(Self::DONE_STR),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TitleError {
    #[error("Todo title cannot be empty")]
    Empty,
    #[error("Todo title cannot have more than 64 characters")]
    Length,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DescriptionError {
    #[error("Todo description cannot have more than 64 characters")]
    Length,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error("Todo status must be one the following values: todo, in_progress or done")]
pub struct ParseTodoStatusError;
