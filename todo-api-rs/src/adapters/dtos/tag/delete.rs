use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug, Default)]
pub struct DeleteTagRequest {
    pub id: Option<String>,
}

impl DeleteTagRequest {
    pub fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidId)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Tag id is required")]
    EmptyId,
    #[error("Invalid tag id format")]
    InvalidId,
}
