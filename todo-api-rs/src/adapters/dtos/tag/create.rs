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
            .filter(|t| !t.is_empty())
            .ok_or(ParseError::EmptyName)
            .and_then(|name| Name::new(name).map_err(ParseError::InvalidName))?;

        let description = Description::new(self.description.filter(|d| !d.is_empty()))
            .map_err(ParseError::InvalidDescription)?;

        Ok(CreateTagInput { name, description })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyName,
    InvalidName(NameError),
    InvalidDescription(DescriptionError),
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyName => "required string".into(),
            Self::InvalidName(err) => err.to_string(),
            Self::InvalidDescription(err) => err.to_string(),
        }
    }
}
