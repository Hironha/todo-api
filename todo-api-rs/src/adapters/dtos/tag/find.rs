use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::find::{FindTagError, FindTagInput};
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

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Finding(FindTagError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing find tag input"),
            Self::Finding(_) => write!(f, "failed finding tag"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
