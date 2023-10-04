use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::tag::TagView;
use crate::application::dtos::tag::find::FindTagInput;
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct RawInput {
    pub id: Option<String>,
}

impl ParsableInput<FindTagInput, ParseError> for RawInput {
    fn parse(self) -> Result<FindTagInput, ParseError> {
        self.id
            .map(|id| Id::parse_str(&id))
            .ok_or(ParseError::EmptyId)?
            .map_err(|_| ParseError::InvalidId)
            .map(FindTagInput)
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
    NotFound,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyId,
    InvalidId,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyId => "required string".into(),
            Self::InvalidId => "invalid id format".into(),
        }
    }
}
