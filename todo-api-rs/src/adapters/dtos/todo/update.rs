use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::dtos::todo::update::UpdateTodoInput;
use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError, TodoEntity};
use crate::domain::types::{Date, Id};

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
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub done: Option<bool>,
}

impl ParsableInput<UpdateTodoInput, ParseError> for RawInput {
    fn parse(self) -> Result<UpdateTodoInput, ParseError> {
        let id = self
            .id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)?;

        let title = self
            .title
            .filter(|t| !t.is_empty())
            .ok_or(ParseError::EmptyTitle)?;

        let description = self.description.filter(|d| !d.is_empty());

        let done = self.done.ok_or(ParseError::EmptyDone)?;

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(UpdateTodoInput {
            id: Id::parse_str(&id).map_err(|_| ParseError::InvalidId)?,
            title: Title::new(title).map_err(ParseError::InvalidTitle)?,
            description: Description::new(description).map_err(ParseError::InvalidDescription)?,
            todo_at,
            done,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Validation(ParseError),
    NotFound,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyId,
    InvalidId,
    EmptyTitle,
    InvalidTitle(TitleError),
    InvalidDescription(DescriptionError),
    TodoAt,
    EmptyDone,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyId => "required string".into(),
            Self::InvalidId => "invalid id format".into(),
            Self::EmptyTitle => "required string".into(),
            Self::InvalidTitle(e) => e.description(),
            Self::InvalidDescription(e) => e.description(),
            Self::TodoAt => {
                "optional string, but if defined, should be a date on YYYY-MM-DD format".into()
            }
            Self::EmptyDone => "required boolean".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParsableInput;

    #[test]
    fn parse_success() {
        let input_schema = super::RawInput {
            id: Some(super::Id::new().to_string()),
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
            done: Some(false),
        };

        assert!(input_schema.parse().is_ok());
    }

    #[test]
    fn parse_id_fail() {
        let none_id_schema = super::RawInput {
            id: None,
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
            done: Some(false),
        };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err_and(|e| e == super::ParseError::EmptyId));

        let invalid_id_schema = super::RawInput {
            id: Some("invalid-id".to_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: None,
            done: Some(false),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err_and(|e| e == super::ParseError::InvalidId));
    }

    #[test]
    fn parse_title_fail() {
        let none_title_schema = super::RawInput {
            id: Some(super::Id::new().to_string()),
            title: None,
            description: None,
            todo_at: None,
            done: Some(false),
        };
        let none_title_input = none_title_schema.parse();

        assert!(none_title_input.is_err_and(|e| e == super::ParseError::EmptyTitle));

        let empty_title_schema = super::RawInput {
            id: Some(super::Id::new().to_string()),
            title: Some("".to_string()),
            description: None,
            todo_at: None,
            done: Some(false),
        };
        let empty_title_input = empty_title_schema.parse();

        assert!(empty_title_input.is_err_and(|e| e == super::ParseError::EmptyTitle));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at_schema = super::RawInput {
            id: Some(super::Id::new().to_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("todo_at".to_string()),
            done: Some(false),
        };
        let invalid_todo_at_input = invalid_todo_at_schema.parse();

        assert!(invalid_todo_at_input.is_err_and(|e| e == super::ParseError::TodoAt))
    }
}
