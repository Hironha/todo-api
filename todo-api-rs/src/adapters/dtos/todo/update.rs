use serde::Serialize;

use crate::domain::types::{Date, Id};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Id,
    Title,
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Id => "required",
            Self::Title => "required",
            Self::TodoAt => "optional, but if defined, should be a date on Y-M-D format",
        };
        description.to_string()
    }
}

#[derive(Debug)]
pub struct InputSchema {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

#[derive(Debug)]
pub struct Input {
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

impl Input {
    pub fn parse(schema: InputSchema) -> Result<Input, ParseError> {
        let id = schema
            .id
            .map(|id| Id::parse_str(&id))
            .ok_or(ParseError::Id)?
            .map_err(|_| ParseError::Id)?;

        let title = schema
            .title
            .filter(|t| !t.is_empty())
            .ok_or(ParseError::Title)?;

        let description = schema.description.filter(|d| !d.is_empty());

        let todo_at = schema
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(Input {
            id,
            title,
            description,
            todo_at,
        })
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
mod tests {
    #[test]
    fn parse_success() {
        let input_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
        };

        assert!(super::Input::parse(input_schema).is_ok());
    }

    #[test]
    fn parse_id_fail() {
        let none_id_schema = super::InputSchema {
            id: None,
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
        };
        let none_id_input = super::Input::parse(none_id_schema);

        assert!(none_id_input.is_err_and(|e| e == super::ParseError::Id));

        let invalid_id_schema = super::InputSchema {
            id: Some("invalid-id".to_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: None,
        };
        let invalid_id_input = super::Input::parse(invalid_id_schema);

        assert!(invalid_id_input.is_err_and(|e| e == super::ParseError::Id));
    }

    #[test]
    fn parse_title_fail() {
        let none_title_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
            title: None,
            description: None,
            todo_at: None,
        };
        let none_title_input = super::Input::parse(none_title_schema);

        assert!(none_title_input.is_err_and(|e| e == super::ParseError::Title));

        let empty_title_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
            title: Some("".to_string()),
            description: None,
            todo_at: None,
        };
        let empty_title_input = super::Input::parse(empty_title_schema);

        assert!(empty_title_input.is_err_and(|e| e == super::ParseError::Title));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("todo_at".to_string()),
        };
        let invalid_todo_at_input = super::Input::parse(invalid_todo_at_schema);

        assert!(invalid_todo_at_input.is_err_and(|e| e == super::ParseError::TodoAt))
    }
}
