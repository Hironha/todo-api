use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput};
use crate::domain::entities::todo::{
    Description, DescriptionError, Title, TitleError, TodoEntityStatus, TodoEntityStatusError,
};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub status: Option<String>,
}

impl Parse<CreateTodoInput, ParseError> for CreateRequest {
    fn parse(self) -> Result<CreateTodoInput, ParseError> {
        let title = self
            .title
            .ok_or(ParseError::EmptyTitle)
            .and_then(|title| Title::new(title).map_err(ParseError::InvalidTitle))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::InvalidDescription)?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::InvalidTodoAt)?;

        let status = self
            .status
            .ok_or(ParseError::EmptyStatus)
            .and_then(|status| {
                TodoEntityStatus::try_from(status.as_str()).map_err(ParseError::InvalidStatus)
            })?;

        Ok(CreateTodoInput {
            title,
            description,
            todo_at,
            status,
        })
    }
}

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Creating(CreateTodoError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing create todo input"),
            Self::Creating(_) => write!(f, "failed creating todo"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parsing(err) => Some(err),
            Self::Creating(err) => Some(err),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyTitle,
    InvalidTitle(TitleError),
    InvalidDescription(DescriptionError),
    InvalidTodoAt,
    EmptyStatus,
    InvalidStatus(TodoEntityStatusError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "required string"),
            Self::InvalidTitle(err) => err.fmt(f),
            Self::InvalidDescription(err) => err.fmt(f),
            Self::InvalidTodoAt => write!(f, "optional UTC date on YYYY-MM_DD format"),
            Self::EmptyStatus => write!(f, "required string"),
            Self::InvalidStatus(err) => err.fmt(f),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTitle(err) => Some(err),
            Self::InvalidDescription(err) => Some(err),
            Self::InvalidStatus(err) => Some(err),
            Self::EmptyTitle | Self::InvalidTodoAt | Self::EmptyStatus => None,
        }
    }
}
