use crate::adapters::dtos::ParsableInput;
use crate::application::dtos::tag::delete::DeleteTagInput;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct RawInput {
    pub id: Option<String>,
}

impl ParsableInput<DeleteTagInput, ParseError> for RawInput {
    fn parse(self) -> Result<DeleteTagInput, ParseError> {
        let id_source = self
            .id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)?;

        Id::parse_str(id_source.as_str())
            .map(DeleteTagInput)
            .map_err(|_| ParseError::InvalidId)
    }
}

#[derive(Clone, Debug)]
pub struct Output(Result<(), RunError>);

impl Output {
    pub const fn ok() -> Self {
        Self(Ok(()))
    }

    pub const fn err(err: RunError) -> Self {
        Self(Err(err))
    }

    pub fn into_result(self) -> Result<(), RunError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    NotFound,
    Internal
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
