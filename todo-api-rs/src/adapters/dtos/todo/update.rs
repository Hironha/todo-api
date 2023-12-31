use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::domain::entities::todo::{
    Description, DescriptionError, ParseTodoStatusError, Title, TitleError, TodoStatus,
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
                TodoStatus::parse_str(status.as_str()).map_err(ParseError::InvalidStatus)
            })?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::InvalidTodoAt)?;

        Ok(UpdateTodoInput {
            id,
            title,
            description,
            todo_at,
            status,
        })
    }
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    Parsing(ParseError),
    #[error(transparent)]
    Updating(UpdateTodoError),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("id is required")]
    EmptyId,
    #[error("invalid id format")]
    InvalidId,
    #[error("title is required")]
    EmptyTitle,
    #[error("invalid title: {0}")]
    InvalidTitle(TitleError),
    #[error("invalid description: {0}")]
    InvalidDescription(DescriptionError),
    #[error("invalid todo at: should be an UTC date on YYYY-MM-DD format")]
    InvalidTodoAt,
    #[error("status is required")]
    EmptyStatus,
    #[error("invalid status: {0}")]
    InvalidStatus(ParseTodoStatusError),
}
