use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::create::CreateTodoInput;
use crate::domain::entities::todo::{
    Description, DescriptionError, ParseTodoStatusError, Title, TitleError, TodoStatus,
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
            .ok_or(ParseError::InvalidTitle(TitleError::Empty))
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
            .ok_or(ParseError::InvalidStatus(ParseTodoStatusError))
            .and_then(|status| {
                TodoStatus::parse_str(status.as_str()).map_err(ParseError::InvalidStatus)
            })?;

        Ok(CreateTodoInput {
            title,
            description,
            todo_at,
            status,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error(transparent)]
    InvalidTitle(TitleError),
    #[error(transparent)]
    InvalidDescription(DescriptionError),
    #[error("Invalid todo at: should be an UTC date on YYYY-MM-DD format")]
    InvalidTodoAt,
    #[error(transparent)]
    InvalidStatus(ParseTodoStatusError),
}
