use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::CreateTodoInput;
use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError};
use crate::domain::types::Date;

#[derive(Debug)]
pub struct Output(Result<TodoView, RunError>);
impl Output {
    pub const fn ok(view: TodoView) -> Self {
        Self(Ok(view))
    }

    pub const fn err(error: RunError) -> Self {
        Self(Err(error))
    }

    pub fn value(self) -> Result<TodoView, RunError> {
        self.0
    }
}

#[derive(Debug)]
pub struct RawInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl ParsableInput<CreateTodoInput, ParseError> for RawInput {
    fn parse(self) -> Result<CreateTodoInput, ParseError> {
        let title = self
            .title
            .filter(|t| !t.is_empty())
            .ok_or(ParseError::EmptyTitle)?;

        let description = self.description.filter(|d| !d.is_empty());

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(CreateTodoInput {
            title: Title::new(title).map_err(ParseError::InvalidTitle)?,
            description: Description::new(description).map_err(ParseError::InvalidDescription)?,
            todo_at,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RunError {
    Parsing(ParseError),
    Internal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    EmptyTitle,
    InvalidTitle(TitleError),
    InvalidDescription(DescriptionError),
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyTitle => "required string".into(),
            Self::InvalidTitle(e) => e.description(),
            Self::InvalidDescription(e) => e.description(),
            Self::TodoAt => {
                "optional string, but, if defined, should be an UTC date on YYYY-MM-DD format"
                    .into()
            }
        }
    }
}
