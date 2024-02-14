use std::error;

use thiserror::Error;

use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::Id;

pub trait FindPresenter {
    type View;
    fn present(&self, response: FindResponse) -> Self::View;
}

#[derive(Debug)]
pub struct FindRequest {
    pub id: Option<String>,
}

impl FindRequest {
    pub fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::Id)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::Id)))
    }
}

pub type FindResponse = Result<TodoEntity, FindResponseError>;

#[derive(Debug, Error)]
pub enum FindResponseError {
    #[error(transparent)]
    Input(ParseError),
    #[error("Todo with id {0} not found")]
    NotFound(Id),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Invalid todo id format")]
    Id,
}

#[cfg(test)]
mod test {

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
        assert_eq!(none_id_input.unwrap_err(), super::ParseError::Id);

        let invalid_id_schema = super::FindRequest {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err());
        assert_eq!(invalid_id_input.unwrap_err(), super::ParseError::Id);
    }
}
