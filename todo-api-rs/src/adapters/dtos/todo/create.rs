use std::error;

use thiserror::Error;

use crate::application::dtos::todo::create::CreateTodoInput;
use crate::domain::entities::todo::{
    Description, DescriptionError, ParseTodoStatusError, Title, TitleError, TodoEntity, TodoStatus,
};
use crate::domain::types::Date;

pub trait CreateTodoPresenter {
    type View;
    fn present(&self, result: Result<TodoEntity, Box<dyn error::Error>>) -> Self::View;
}

#[derive(Clone, Debug)]
pub struct CreateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub status: Option<String>,
}

impl CreateTodoRequest {
    pub fn parse(self) -> Result<CreateTodoInput, ParseError> {
        let title = self
            .title
            .ok_or(ParseError::Title(TitleError::Empty))
            .and_then(|title| Title::new(title).map_err(ParseError::Title))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::Description)?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        let status = self
            .status
            .ok_or(ParseError::Status(ParseTodoStatusError))
            .and_then(|status| {
                TodoStatus::parse_str(status.as_str()).map_err(ParseError::Status)
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
    Title(TitleError),
    #[error(transparent)]
    Description(DescriptionError),
    #[error("Invalid todo at: should be an UTC date on YYYY-MM-DD format")]
    TodoAt,
    #[error(transparent)]
    Status(ParseTodoStatusError),
}
