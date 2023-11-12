use std::error::Error;
use std::fmt;

use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput};
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

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Updating(UpdateTagError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing update tag input"),
            Self::Updating(_) => write!(f, "failed updating tag"),
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
    EmptyName,
    InvalidName(NameError),
    InvalidDescription(DescriptionError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => write!(f, "required string"),
            Self::InvalidId => write!(f, "invalid id format"),
            Self::EmptyName => write!(f, "required string"),
            Self::InvalidName(err) => err.fmt(f),
            Self::InvalidDescription(err) => err.fmt(f),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyId | Self::InvalidId | Self::EmptyName => None,
            Self::InvalidName(err) => Some(err),
            Self::InvalidDescription(err) => Some(err),
        }
    }
}
