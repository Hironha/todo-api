use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::delete::DeleteTodoInput;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteRequest {
    pub id: Option<String>,
}

impl Parse<DeleteTodoInput, ParseError> for DeleteRequest {
    fn parse(self) -> Result<DeleteTodoInput, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidId)))
            .map(DeleteTodoInput)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    TodoNotFound,
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
            Self::InvalidId => "invalid id format".into(),
            Self::EmptyId => "required string".into(),
        }
    }
}
