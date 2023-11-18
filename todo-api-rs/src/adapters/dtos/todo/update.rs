use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError};
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug)]
pub struct UpdateRequest {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub done: Option<bool>,
}

impl Parse<UpdateTodoInput, ParseError> for UpdateRequest {
    fn parse(self) -> Result<UpdateTodoInput, ParseError> {
        let id = self
            .id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)
            .map(|id| Id::parse_str(&id))?
            .or(Err(ParseError::InvalidId))?;

        let title = self
            .title
            .ok_or(ParseError::EmptyTitle)
            .and_then(|t| Title::new(t).map_err(ParseError::InvalidTitle))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::InvalidDescription)?;

        let done = self.done.ok_or(ParseError::EmptyDone)?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(UpdateTodoInput {
            id,
            title,
            description,
            todo_at,
            done,
        })
    }
}

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Updating(UpdateTodoError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing update todo input"),
            Self::Updating(_) => write!(f, "failed updating todo"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parsing(err) => Some(err),
            Self::Updating(err) => Some(err),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyId,
    InvalidId,
    EmptyTitle,
    InvalidTitle(TitleError),
    InvalidDescription(DescriptionError),
    TodoAt,
    EmptyDone,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId | Self::EmptyTitle => write!(f, "required string"),
            Self::EmptyDone => write!(f, "required boolean"),
            Self::InvalidId => write!(f, "invalid id format"),
            Self::TodoAt => write!(f, "optional UTC date on YYYY-MM_DD format"),
            Self::InvalidTitle(err) => err.fmt(f),
            Self::InvalidDescription(err) => err.fmt(f),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDescription(err) => Some(err),
            Self::InvalidTitle(err) => Some(err),
            Self::EmptyDone | Self::EmptyId | Self::EmptyTitle | Self::InvalidId | Self::TodoAt => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parse;

    #[test]
    fn parse_success() {
        let input_schema = super::UpdateRequest {
            id: Some(super::Id::new().to_string()),
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
            done: Some(false),
        };

        assert!(input_schema.parse().is_ok());
    }

    #[test]
    fn parse_id_fail() {
        let none_id_schema = super::UpdateRequest {
            id: None,
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
            done: Some(false),
        };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err_and(|e| e == super::ParseError::EmptyId));

        let invalid_id_schema = super::UpdateRequest {
            id: Some("invalid-id".to_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: None,
            done: Some(false),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err_and(|e| e == super::ParseError::InvalidId));
    }

    #[test]
    fn parse_title_fail() {
        let none_title_schema = super::UpdateRequest {
            id: Some(super::Id::new().to_string()),
            title: None,
            description: None,
            todo_at: None,
            done: Some(false),
        };
        let none_title_input = none_title_schema.parse();

        assert!(none_title_input.is_err_and(|e| e == super::ParseError::EmptyTitle));

        let empty_title_schema = super::UpdateRequest {
            id: Some(super::Id::new().to_string()),
            title: Some("".to_string()),
            description: None,
            todo_at: None,
            done: Some(false),
        };
        let empty_title_input = empty_title_schema.parse();

        assert!(empty_title_input.is_err_and(|e| e == super::ParseError::EmptyTitle));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at_schema = super::UpdateRequest {
            id: Some(super::Id::new().to_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("todo_at".to_string()),
            done: Some(false),
        };
        let invalid_todo_at_input = invalid_todo_at_schema.parse();

        assert!(invalid_todo_at_input.is_err_and(|e| e == super::ParseError::TodoAt))
    }
}
