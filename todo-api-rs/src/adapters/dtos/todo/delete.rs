use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteRequest {
    pub id: Option<String>,
}

impl DeleteRequest {
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
