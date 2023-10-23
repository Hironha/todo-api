use crate::adapters::dtos::Parse;
use crate::adapters::views::todo::TodoView;
use crate::application::dtos::todo::create::CreateTodoInput;
use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError, TodoEntity};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct Output(Result<TodoView, RunError>);
impl Output {
    pub fn from_todo(todo: TodoEntity) -> Self {
        Self(Ok(TodoView::from(todo)))
    }

    pub const fn err(error: RunError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<TodoView, RunError> {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct RawInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl Parse<CreateTodoInput, ParseError> for RawInput {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
