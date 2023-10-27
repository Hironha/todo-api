use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::find::FindTagInput;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    NotFound,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyId,
    InvalidId,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyId => "required string".into(),
            Self::InvalidId => "invalid id format".into(),
        }
    }
}
