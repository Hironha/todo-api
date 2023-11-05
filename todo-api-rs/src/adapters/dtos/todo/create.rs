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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyTitle,
    InvalidTitle(TitleError),
    InvalidDescription(DescriptionError),
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyTitle => "required string".into(),
            Self::InvalidTitle(err) => err.to_string(),
            Self::InvalidDescription(err) => err.to_string(),
            Self::TodoAt => {
                "optional string, but, if defined, should be an UTC date on YYYY-MM-DD format"
                    .into()
            }
        }
    }
}
