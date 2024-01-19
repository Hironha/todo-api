use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug, Default)]
pub struct FindRequest {
    pub id: Option<String>,
}

impl FindRequest {
    pub fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).map_err(|_| ParseError::InvalidId))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Tag id is required")]
    EmptyId,
    #[error("Invalid tag id format")]
    InvalidId,
}
