use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::update::UpdateTagInput;
use crate::domain::entities::tag::{Description, DescriptionError, Name, NameError};
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct UpdateRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Parse<UpdateTagInput, ParseError> for UpdateRequest {
    fn parse(self) -> Result<UpdateTagInput, ParseError> {
        let id = self
            .id
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).map_err(|_| ParseError::InvalidId))?;

        let name = self
            .name
            .ok_or(ParseError::EmptyName)
            .and_then(|name| Name::new(name).map_err(ParseError::InvalidName))?;

        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(ParseError::InvalidDescription)?;

        Ok(UpdateTagInput {
            id,
            name,
            description,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("id is required")]
    EmptyId,
    #[error("invalid id format")]
    InvalidId,
    #[error("name is required")]
    EmptyName,
    #[error("invalid name: {0}")]
    InvalidName(NameError),
    #[error("invalid description: {0}")]
    InvalidDescription(DescriptionError),
}
