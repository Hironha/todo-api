use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::create::CreateTagInput;
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
            .ok_or(ParseError::InvalidName(NameError::Empty))
            .and_then(|name| Name::new(name).map_err(ParseError::InvalidName))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::InvalidDescription)?;

        Ok(CreateTagInput { name, description })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error(transparent)]
    InvalidName(NameError),
    #[error(transparent)]
    InvalidDescription(DescriptionError),
}
