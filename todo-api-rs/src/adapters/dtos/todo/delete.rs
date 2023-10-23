use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::delete::DeleteTodoInput;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct Output(Result<(), RunError>);

impl Output {
    pub const fn ok() -> Self {
        Self(Ok(()))
    }

    pub const fn err(error: RunError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<(), RunError> {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct RawInput {
    pub id: Option<String>,
}

impl Parse<DeleteTodoInput, ParseError> for RawInput {
    fn parse(self) -> Result<DeleteTodoInput, ParseError> {
        let id = self
            .id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)?;

        Id::parse_str(&id)
            .map_err(|_| ParseError::InvalidId)
            .map(DeleteTodoInput::new)
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
