use std::fmt;

use thiserror::Error;

use crate::domain::types::{Date, DateTime, Id};

#[derive(Clone, Debug)]
pub struct TodoEntity {
    id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: TodoStatus,
    pub todo_at: Option<Date>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl TodoEntity {
    pub fn create() -> TodoEntityBuilder<Id, (), ()> {
        TodoEntityBuilder::<(), (), ()>::new().id(Id::new())
    }

    pub fn builder() -> TodoEntityBuilder<(), (), ()> {
        TodoEntityBuilder::<(), (), ()>::new()
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn created_at(&self) -> Option<DateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<DateTime> {
        self.updated_at
    }
}

#[derive(Clone, Debug)]
pub struct TodoEntityBuilder<I, T, S> {
    id: I,
    title: T,
    description: Option<Description>,
    status: S,
    todo_at: Option<Date>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl<I, T, S> TodoEntityBuilder<I, T, S> {
    pub fn new() -> TodoEntityBuilder<(), (), ()> {
        TodoEntityBuilder::<(), (), ()> {
            id: (),
            title: (),
            description: None,
            status: (),
            todo_at: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl<T, S> TodoEntityBuilder<(), T, S> {
    pub fn id(self, id: Id) -> TodoEntityBuilder<Id, T, S> {
        TodoEntityBuilder::<Id, T, S> {
            id,
            title: self.title,
            description: self.description,
            status: self.status,
            todo_at: self.todo_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl<I, S> TodoEntityBuilder<I, (), S> {
    pub fn title(self, title: Title) -> TodoEntityBuilder<I, Title, S> {
        TodoEntityBuilder::<I, Title, S> {
            id: self.id,
            title,
            description: self.description,
            status: self.status,
            todo_at: self.todo_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl<I, T> TodoEntityBuilder<I, T, ()> {
    pub fn status(self, status: TodoStatus) -> TodoEntityBuilder<I, T, TodoStatus> {
        TodoEntityBuilder::<I, T, TodoStatus> {
            id: self.id,
            title: self.title,
            description: self.description,
            status,
            todo_at: self.todo_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl TodoEntityBuilder<Id, Title, TodoStatus> {
    pub fn build(self) -> TodoEntity {
        TodoEntity {
            id: self.id,
            title: self.title,
            description: self.description,
            status: self.status,
            todo_at: self.todo_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn description(mut self, description: Option<Description>) -> Self {
        self.description = description;
        self
    }

    pub fn todo_at(mut self, todo_at: Option<Date>) -> Self {
        self.todo_at = todo_at;
        self
    }

    pub fn created_at(mut self, created_at: Option<DateTime>) -> Self {
        self.created_at = created_at;
        self
    }

    pub fn updated_at(mut self, updated_at: Option<DateTime>) -> Self {
        self.updated_at = updated_at;
        self
    }
}

impl PartialEq for TodoEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TodoEntity {}

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

    pub fn as_str(&self) -> &str {
        &self.0
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
#[error(
    "Todo status must be one the following values: {}, {}, {}",
    TodoStatus::TODO_STR,
    TodoStatus::IN_PROGRESS_STR,
    TodoStatus::DONE_STR
)]
pub struct ParseTodoStatusError;
