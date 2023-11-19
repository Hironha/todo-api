use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::domain::entities::todo::{
    Description, DescriptionError, TodoEntityStatusError, Title, TitleError, TodoEntityStatus,
};
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug)]
pub struct UpdateRequest {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub status: Option<String>,
}

impl Parse<UpdateTodoInput, ParseError> for UpdateRequest {
    fn parse(self) -> Result<UpdateTodoInput, ParseError> {
        let id = self
            .id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .map(|id| Id::parse_str(&id))?
            .or(Err(ParseError::InvalidId))?;

        let title = self
            .title
            .ok_or(ParseError::EmptyTitle)
            .and_then(|t| Title::new(t).map_err(ParseError::InvalidTitle))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::InvalidDescription)?;

        let status = self
            .status
            .ok_or(ParseError::EmptyStatus)
            .and_then(|status| {
                TodoEntityStatus::try_from(status.as_str()).map_err(ParseError::InvalidStatus)
            })?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(UpdateTodoInput {
            id,
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
    Updating(UpdateTodoError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing update todo input"),
            Self::Updating(_) => write!(f, "failed updating todo"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parsing(err) => Some(err),
            Self::Updating(err) => Some(err),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyId,
    InvalidId,
    EmptyTitle,
    InvalidTitle(TitleError),
    InvalidDescription(DescriptionError),
    TodoAt,
    EmptyStatus,
    InvalidStatus(TodoEntityStatusError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId | Self::EmptyTitle => write!(f, "required string"),
            Self::EmptyStatus => write!(f, "required string"),
            Self::InvalidStatus(err) => err.fmt(f),
            Self::InvalidId => write!(f, "invalid id format"),
            Self::TodoAt => write!(f, "optional UTC date on YYYY-MM_DD format"),
            Self::InvalidTitle(err) => err.fmt(f),
            Self::InvalidDescription(err) => err.fmt(f),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDescription(err) => Some(err),
            Self::InvalidTitle(err) => Some(err),
            Self::InvalidStatus(err) => Some(err),
            Self::EmptyStatus
            | Self::EmptyId
            | Self::EmptyTitle
            | Self::InvalidId
            | Self::TodoAt => None,
        }
    }
}
