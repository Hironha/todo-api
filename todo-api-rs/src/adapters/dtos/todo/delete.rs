use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::delete::{DeleteTodoError, DeleteTodoInput};
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

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Deleting(DeleteTodoError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing delete input"),
            Self::Deleting(_) => write!(f, "failed deleting todo"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parsing(err) => Some(err),
            Self::Deleting(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    EmptyId,
    InvalidId,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => write!(f, "required string"),
            Self::InvalidId => write!(f, "invalid id format"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyId | Self::InvalidId => None,
        }
    }
}
