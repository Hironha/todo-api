use crate::domain::types::Id;
use serde::Serialize;

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
pub struct InputSchema {
    pub id: Option<String>,
}

#[derive(Debug)]
pub struct Input {
    pub id: Id,
}

impl Input {
    pub fn parse(schema: InputSchema) -> Result<Input, ParseError> {
        let id = schema
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
    #[test]
    fn parse_success() {
        let input_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
        };

        assert!(super::Input::parse(input_schema).is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id_schema = super::InputSchema { id: None };
        let none_id_payload = super::Input::parse(none_id_schema);

        assert!(none_id_payload.is_err());
        assert_eq!(none_id_payload.unwrap_err(), super::ParseError::Id);

        let invalid_id_schema = super::InputSchema {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_payload = super::Input::parse(invalid_id_schema);

        assert!(invalid_id_payload.is_err());
        assert_eq!(invalid_id_payload.unwrap_err(), super::ParseError::Id);
    }
}
