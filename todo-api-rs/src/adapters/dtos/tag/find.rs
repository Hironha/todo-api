use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::find::FindTagError;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindRequest {
    pub id: Option<String>,
}

impl Parse<Id, ParseError> for FindRequest {
    fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).map_err(|_| ParseError::InvalidId))
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
