use std::error;

use thiserror::Error;

use crate::application::dtos::todo::create::CreateTodoInput;
use crate::domain::entities::todo::{
    Description, DescriptionError, StatusError, Status, Title, TitleError, TodoEntity,
};
use crate::domain::types::{Date, ParseDateError};

pub trait CreatePresenter {
    type View;
    fn present(&self, response: CreateResponse) -> Self::View;
}

#[derive(Clone, Debug)]
pub struct CreateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub status: Option<String>,
}

impl CreateRequest {
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
            .map_err(ParseError::TodoAt)?;

        let status = self
            .status
            .ok_or(ParseError::Status(StatusError))
            .and_then(|status| Status::parse_str(status.as_str()).map_err(ParseError::Status))?;

        Ok(CreateTodoInput {
            title,
            description,
            todo_at,
            status,
        })
    }
}

pub type CreateResponse = Result<TodoEntity, CreateResponseError>;

#[derive(Debug, Error)]
pub enum CreateResponseError {
    #[error(transparent)]
    Input(ParseError),
    #[error("Todo with title {} already exists", 0.to_string())]
    DuplicatedTitle(Title),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error(transparent)]
    Title(TitleError),
    #[error(transparent)]
    Description(DescriptionError),
    #[error(transparent)]
    TodoAt(ParseDateError),
    #[error(transparent)]
    Status(StatusError),
}
