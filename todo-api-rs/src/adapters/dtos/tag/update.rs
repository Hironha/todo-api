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

        let description =
            Description::new(self.description).map_err(ParseError::InvalidDescription)?;

        Ok(UpdateTagInput {
            id,
            name,
            description,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    NotFound,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyId,
    InvalidId,
    EmptyName,
    InvalidName(NameError),
    InvalidDescription(DescriptionError),
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyId => "required string".into(),
            Self::InvalidId => "invalid id format".into(),
            Self::EmptyName => "required string".into(),
            Self::InvalidName(err) => err.description(),
            Self::InvalidDescription(err) => err.description(),
        }
    }
}
