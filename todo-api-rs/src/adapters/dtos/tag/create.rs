use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::create::{CreateTagError, CreateTagInput};
use crate::domain::entities::tag::{Description, DescriptionError, Name, NameError};

#[derive(Clone, Debug)]
pub struct CreateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Parse<CreateTagInput, ParseError> for CreateRequest {
    fn parse(self) -> Result<CreateTagInput, ParseError> {
        let name = self
            .name
            .ok_or(ParseError::EmptyName)
            .and_then(|name| Name::new(name).map_err(ParseError::InvalidName))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::InvalidDescription)?;

        Ok(CreateTagInput { name, description })
    }
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    Parsing(ParseError),
    #[error(transparent)]
    Creating(CreateTagError),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("name is required")]
    EmptyName,
    #[error("invalid name: {0}")]
    InvalidName(NameError),
    #[error("invalid description: {0}")]
    InvalidDescription(DescriptionError),
}
