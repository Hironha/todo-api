use thiserror::Error;

use crate::domain::types::Id;

#[derive(Debug)]
pub struct FindTodoRequest {
    pub id: Option<String>,
}

impl FindTodoRequest {
    pub fn parse(self) -> Result<Id, ParseError> {
        self.id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidId)))
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Todo id is required")]
    EmptyId,
    #[error("Invalid id format")]
    InvalidId,
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_success() {
        let input_schema = super::FindTodoRequest {
            id: Some(super::Id::new().to_string()),
        };

        assert!(input_schema.parse().is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id_schema = super::FindTodoRequest { id: None };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err());
        assert_eq!(none_id_input.unwrap_err(), super::ParseError::EmptyId);

        let invalid_id_schema = super::FindTodoRequest {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err());
        assert_eq!(invalid_id_input.unwrap_err(), super::ParseError::InvalidId);
    }
}
