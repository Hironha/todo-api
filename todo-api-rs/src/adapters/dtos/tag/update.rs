use thiserror::Error;

use crate::application::dtos::tag::update::UpdateTagInput;
use crate::domain::entities::tag::{Description, DescriptionError, Name, NameError};
use crate::domain::types::Id;

#[derive(Clone, Debug, Default)]
pub struct UpdateTagRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateTagRequest {
    pub fn parse(self) -> Result<UpdateTagInput, ParseError> {
        let id = self
            .id
            .ok_or(ParseError::EmptyId)
            .and_then(|id| Id::parse_str(&id).map_err(|_| ParseError::InvalidId))?;

        let name = self
            .name
            .ok_or(ParseError::InvalidName(NameError::Empty))
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
    #[error("Tag id is required")]
    EmptyId,
    #[error("Invalid tag id format")]
    InvalidId,
    #[error(transparent)]
    InvalidName(NameError),
    #[error(transparent)]
    InvalidDescription(DescriptionError),
}
