use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::delete::DeleteTagInput;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteRequest {
    pub id: Option<String>,
}

impl Parse<DeleteTagInput, ParseError> for DeleteRequest {
    fn parse(self) -> Result<DeleteTagInput, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidId)))
            .map(DeleteTagInput)
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
