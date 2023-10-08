use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::tag::TagView;
use crate::application::dtos::tag::create::CreateTagInput;
use crate::domain::entities::tag::{Description, DescriptionError, Name, NameError, TagEntity};

#[derive(Clone, Debug)]
pub struct RawInput {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl ParsableInput<CreateTagInput, ParseError> for RawInput {
    fn parse(self) -> Result<CreateTagInput, ParseError> {
        let name = self
            .name
            .filter(|t| !t.is_empty())
            .map(Name::new)
            .ok_or(ParseError::EmptyName)?
            .map_err(ParseError::InvalidName)?;

        let description = Description::new(self.description.filter(|d| !d.is_empty()))
            .map_err(ParseError::InvalidDescription)?;

        Ok(CreateTagInput { name, description })
    }
}

#[derive(Clone, Debug)]
pub struct Output(Result<TagView, RunError>);

impl Output {
    pub const fn err(error: RunError) -> Self {
        Self(Err(error))
    }

    pub fn from_tag(tag: TagEntity) -> Self {
        Self(Ok(TagView::from(tag)))
    }

    pub fn into_result(self) -> Result<TagView, RunError> {
        self.0
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
            Self::InvalidName(err) => err.description(),
            Self::InvalidDescription(err) => err.description(),
        }
    }
}
