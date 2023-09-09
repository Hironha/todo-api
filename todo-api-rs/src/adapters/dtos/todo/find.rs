use serde::Serialize;

use crate::adapters::dtos::ParsableInput;
use crate::domain::types::Id;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Id,
}

impl ParseError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Id => "required",
        };
        description.to_string()
    }
}

#[derive(Debug)]
pub struct Input {
    pub id: Id,
}

#[derive(Debug)]
pub struct InputSchema {
    pub id: Option<String>,
}

impl ParsableInput<Input, ParseError> for InputSchema {
    fn parse(self) -> Result<Input, ParseError> {
        let id = self
            .id
            .map(|id| Id::parse_str(&id))
            .ok_or(ParseError::Id)?
            .map_err(|_| ParseError::Id)?;

        Ok(Input { id })
    }
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    /// None or Date stringified on Y-M-D format
    #[serde(rename(serialize = "createdAt"))]
    pub todo_at: Option<String>,
    /// Date stringified on `RFC-3339` format
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
    /// Date stringified on `RFC-3339` format
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
}

#[cfg(test)]
mod test {
    use super::ParsableInput;

    #[test]
    fn parse_success() {
        let input_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
        };

        assert!(input_schema.parse().is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id_schema = super::InputSchema { id: None };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err());
        assert_eq!(none_id_input.unwrap_err(), super::ParseError::Id);

        let invalid_id_schema = super::InputSchema {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err());
        assert_eq!(invalid_id_input.unwrap_err(), super::ParseError::Id);
    }
}
