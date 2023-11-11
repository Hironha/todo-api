use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::create::CreateTodoInput;
use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl Parse<CreateTodoInput, ParseError> for CreateRequest {
    fn parse(self) -> Result<CreateTodoInput, ParseError> {
        let title = self
            .title
            .filter(|t| !t.is_empty())
            .ok_or(ParseError::EmptyTitle)
            .and_then(|title| Title::new(title).map_err(ParseError::InvalidTitle))?;

        let description = Description::new(self.description.filter(|d| !d.is_empty()))
            .map_err(ParseError::InvalidDescription)?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(CreateTodoInput {
            title,
            description,
            todo_at,
        })
    }
}

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Repository(Box<dyn Error>),
}

#[derive(Debug)]
pub enum ParseError {
    EmptyTitle,
    InvalidTitle(TitleError),
    InvalidDescription(DescriptionError),
    TodoAt,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "required string"),
            Self::InvalidTitle(err) => err.fmt(f),
            Self::InvalidDescription(err) => err.fmt(f),
            Self::TodoAt => write!(f, "optional UTC date on YYYY-MM_DD format"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyTitle | Self::TodoAt => None,
            Self::InvalidTitle(err) => Some(err),
            Self::InvalidDescription(err) => Some(err),
        }
    }
}
