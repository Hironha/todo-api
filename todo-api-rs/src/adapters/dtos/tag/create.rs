use std::error::Error;
use std::fmt;

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

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Creating(CreateTagError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing create tag input"),
            Self::Creating(_) => write!(f, "failed creating tag"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parsing(err) => Some(err),
            Self::Creating(err) => Some(err),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyName,
    InvalidName(NameError),
    InvalidDescription(DescriptionError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "required string"),
            Self::InvalidName(err) => err.fmt(f),
            Self::InvalidDescription(err) => err.fmt(f),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyName => None,
            Self::InvalidName(err) => Some(err),
            Self::InvalidDescription(err) => Some(err),
        }
    }
}
