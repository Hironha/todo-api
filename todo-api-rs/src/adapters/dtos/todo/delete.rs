use std::error;

use thiserror::Error;

use crate::domain::types::Id;

pub trait DeleteTodoPresenter {
    type View;
    fn present(&self, result: Result<(), Box<dyn error::Error>>) -> Self::View;
}

#[derive(Clone, Debug)]
pub struct DeleteTodoRequest {
    pub id: Option<String>,
}

impl DeleteTodoRequest {
    pub fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidId)))
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Todo id is required")]
    EmptyId,
    #[error("Invalid id format")]
    InvalidId,
}
