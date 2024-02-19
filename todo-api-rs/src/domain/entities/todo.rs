use std::fmt;

use thiserror::Error;

use crate::domain::types::{Date, DateTime, Id};

#[derive(Clone, Debug)]
pub struct TodoEntity {
    id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: Status,
    pub todo_at: Option<Date>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl TodoEntity {
    pub fn new(props: NewProps) -> Self {
        Self {
            id: Id::new(),
            title: props.title,
            description: props.description,
            status: props.status,
            todo_at: props.todo_at,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn init(props: InitProps) -> Self {
        Self {
            id: props.id,
            title: props.title,
            description: props.description,
            status: props.status,
            todo_at: props.todo_at,
            created_at: props.created_at,
            updated_at: props.updated_at,
        }
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
pub struct NewProps {
    pub title: Title,
    pub description: Option<Description>,
    pub status: Status,
    pub todo_at: Option<Date>,
}

#[derive(Clone, Debug)]
pub struct InitProps {
    pub id: Id,
    pub title: Title,
    pub description: Option<Description>,
    pub status: Status,
    pub todo_at: Option<Date>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
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

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    const TODO_STR: &'static str = "todo";
    const IN_PROGRESS_STR: &'static str = "in_progress";
    const DONE_STR: &'static str = "done";

    pub fn parse_str(value: &str) -> Result<Self, StatusError> {
        match value {
            Self::TODO_STR => Ok(Self::Todo),
            Self::IN_PROGRESS_STR => Ok(Self::InProgress),
            Self::DONE_STR => Ok(Self::Done),
            _ => Err(StatusError),
        }
    }
}

impl fmt::Display for Status {
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
    Status::TODO_STR,
    Status::IN_PROGRESS_STR,
    Status::DONE_STR
)]
pub struct StatusError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_title_fails() {
        let empty_title = Title::new("");
        assert_eq!(empty_title, Err(TitleError::Empty));
    }

    #[test]
    fn title_too_big_fails() {
        let src =
            "This is a really big title so it should not satisfy title max length constraints";
        let big_title = Title::new(src);
        assert!(src.len() > Title::MAX_LENGTH);
        assert_eq!(big_title, Err(TitleError::Length));
    }

    #[test]
    fn new_title_works() {
        let src = "Title";
        let title = Title::new(src);
        assert!(title.is_ok());
        assert_eq!(Ok(src), title.as_ref().map(Title::as_str))
    }

    #[test]
    fn title_formats_to_string() {
        let src = String::from("Title");
        let title = Title::new(&src);
        assert_eq!(Ok(src.as_str()), title.as_ref().map(Title::as_str));
        assert_eq!(Ok(src), title.as_ref().map(Title::to_string));
    }

    #[test]
    fn description_too_big_fails() {
        let src = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus euismod, nisl eu pretium aliquet, nisi dolor molestie libero, vel euismod nisi lorem vel justo. Integer pharetra ornare justo, sed rhoncus odio. Etiam bibendum nisl lacus, vulputate iaculis orci placerat ut. Aenean ullamcorper nisl sit amet ullamcorper euismod. Pellentesque eleifend dictum arcu quis tempus.");
        let description = Description::new(&src);
        assert!(src.len() > Description::MAX_LENGTH);
        assert_eq!(Err(DescriptionError::Length), description);
    }

    #[test]
    fn new_description_works() {
        let src = String::from("This is a description text");
        let description = Description::new(&src);
        assert!(description.is_ok());
        assert_eq!(
            Ok(src.as_str()),
            description.as_ref().map(Description::as_str),
        );
    }

    #[test]
    fn description_formats_to_string() {
        let src = String::from("This is a description text");
        let description = Description::new(&src);
        assert_eq!(Ok(src), description.as_ref().map(Description::to_string));
    }

    #[test]
    fn parse_status_from_str_works() {
        let todo = Status::parse_str("todo");
        assert_eq!(Ok(Status::Todo), todo);

        let in_progress = Status::parse_str("in_progress");
        assert_eq!(Ok(Status::InProgress), in_progress);

        let done = Status::parse_str("done");
        assert_eq!(Ok(Status::Done), done);

        let invalid = Status::parse_str("not_status");
        assert_eq!(Err(StatusError), invalid);
    }

    #[test]
    fn status_formats_to_string() {
        let srcs = ["todo", "in_progress", "done"];
        for src in srcs {
            let status = Status::parse_str(src);
            assert_eq!(Ok(src), status.as_ref().map(Status::to_string).as_deref());
        }
    }
}
