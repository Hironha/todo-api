use std::error;

use thiserror::Error;

use crate::domain::types::Id;

pub trait DeletePresenter {
    type View;
    fn present(&self, response: DeleteResponse) -> Self::View;
}

#[derive(Clone, Debug)]
pub struct DeleteRequest {
    pub id: Option<String>,
}

impl DeleteRequest {
    pub fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::Id)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::Id)))
    }
}

pub type DeleteResponse = Result<(), DeleteResponseError>;

#[derive(Debug, Error)]
pub enum DeleteResponseError {
    #[error(transparent)]
    Input(ParseError),
    #[error("Todo with id {0} not found")]
    NotFound(Id),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Invalid todo id format")]
    Id,
}
