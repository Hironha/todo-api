use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::delete::DeleteTagError;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteRequest {
    pub id: Option<String>,
}

impl Parse<Id, ParseError> for DeleteRequest {
    fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidId)))
    }
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    Parsing(ParseError),
    #[error(transparent)]
    Deleting(DeleteTagError),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("id is required")]
    EmptyId,
    #[error("invalid id format")]
    InvalidId,
}
