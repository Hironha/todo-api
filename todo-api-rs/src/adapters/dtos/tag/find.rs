use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::find::{FindTagError, FindTagInput};
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindRequest {
    pub id: Option<String>,
}

impl Parse<FindTagInput, ParseError> for FindRequest {
    fn parse(self) -> Result<FindTagInput, ParseError> {
        self.id
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).map_err(|_| ParseError::InvalidId))
            .map(FindTagInput)
    }
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    Parsing(ParseError),
    #[error(transparent)]
    Finding(FindTagError),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("id is required")]
    EmptyId,
    #[error("invalid id format")]
    InvalidId,
}
