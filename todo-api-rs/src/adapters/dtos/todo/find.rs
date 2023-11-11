use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::find::{FindTodoError, FindTodoInput};
use crate::domain::types::Id;

#[derive(Debug)]
pub struct FindRequest {
    pub id: Option<String>,
}

impl Parse<FindTodoInput, ParseError> for FindRequest {
    fn parse(self) -> Result<FindTodoInput, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidId)))
            .map(FindTodoInput)
    }
}

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Finding(FindTodoError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing find input"),
            Self::Finding(_) => write!(f, "failed finding todo"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod test {
    use super::Parse;

    #[test]
    fn parse_success() {
        let input_schema = super::FindRequest {
            id: Some(super::Id::new().to_string()),
        };

        assert!(input_schema.parse().is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id_schema = super::FindRequest { id: None };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err());
        assert_eq!(none_id_input.unwrap_err(), super::ParseError::EmptyId);

        let invalid_id_schema = super::FindRequest {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err());
        assert_eq!(invalid_id_input.unwrap_err(), super::ParseError::InvalidId);
    }
}
