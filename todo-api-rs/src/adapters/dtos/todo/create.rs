use serde::Serialize;

use crate::domain::types::Date;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Title,
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Title => "required",
            Self::TodoAt => "optional, but if defined, should be a date on Y-M-D format",
        };
        description.to_string()
    }
}

#[derive(Debug)]
pub struct InputSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

#[derive(Debug)]
pub struct Input {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

impl Input {
    pub fn parse(schema: InputSchema) -> Result<Input, ParseError> {
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
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-11".to_string()),
        };

        assert!(super::Input::parse(input_schema).is_ok());
    }

    #[test]
    fn parse_empty_description_to_none() {
        let input_schema = super::InputSchema {
            title: Some("title".to_string()),
            description: Some("".to_string()),
            todo_at: None,
        };
        let payload = super::Input::parse(input_schema);

        assert!(payload.is_ok_and(|p| p.description.is_none()));
    }

    #[test]
    fn parse_title_fail() {
        let none_title_schema = super::InputSchema {
            title: None,
            description: Some("description".to_string()),
            todo_at: None,
        };
        let none_title_payload = super::Input::parse(none_title_schema);

        assert!(none_title_payload.is_err_and(|e| e == super::ParseError::Title));

        let empty_title_schema = super::InputSchema {
            title: Some("".to_string()),
            description: None,
            todo_at: None,
        };
        let empty_title_payload = super::Input::parse(empty_title_schema);

        assert!(empty_title_payload.is_err_and(|e| e == super::ParseError::Title));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at_schema = super::InputSchema {
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("2023-2023-2023".to_string()),
        };

        let invalid_todo_at_payload = super::Input::parse(invalid_todo_at_schema);

        assert!(invalid_todo_at_payload.is_err_and(|e| e == super::ParseError::TodoAt));
    }
}
