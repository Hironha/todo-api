use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::find::FindTodoInput;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Validation(ParseError),
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
