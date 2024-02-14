use std::error;

use thiserror::Error;

use crate::application::dtos::todo::update::UpdateTodoInput;
use crate::domain::entities::todo::{
    Description, DescriptionError, Status, StatusError, Title, TitleError,
};
use crate::domain::types::{Date, Id, ParseDateError};

pub trait UpdatePresenter {
    type View;
    fn present(&self, resposne: UpdateResponse) -> Self::View;
}

#[derive(Clone, Debug)]
pub struct UpdateRequest {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub status: Option<String>,
}

impl UpdateRequest {
    pub fn parse(self) -> Result<UpdateTodoInput, ParseError> {
        let id = self
            .id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::Id)
            .map(|id| Id::parse_str(&id))?
            .or(Err(ParseError::Id))?;

        let title = self
            .title
            .ok_or(ParseError::Title(TitleError::Empty))
            .and_then(|t| Title::new(t).map_err(ParseError::Title))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::Description)?;

        let status = self
            .status
            .ok_or(ParseError::Status(StatusError))
            .and_then(|status| Status::parse_str(status.as_str()).map_err(ParseError::Status))?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(ParseError::TodoAt)?;

        Ok(UpdateTodoInput {
            id,
            title,
            description,
            todo_at,
            status,
        })
    }
}

pub type UpdateResponse = Result<(), UpdateResponseError>;

#[derive(Debug, Error)]
pub enum UpdateResponseError {
    #[error(transparent)]
    Input(ParseError),
    #[error("Todo with id {0} not found")]
    NotFound(Id),
    #[error("Todo with title {0} already exists")]
    DuplicatedTitle(Title),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Invalid todo id format")]
    Id,
    #[error(transparent)]
    Title(TitleError),
    #[error(transparent)]
    Description(DescriptionError),
    #[error(transparent)]
    TodoAt(ParseDateError),
    #[error(transparent)]
    Status(StatusError),
}
