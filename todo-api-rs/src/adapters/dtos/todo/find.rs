use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::FindTodoInput;
use crate::domain::types::Id;

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
pub struct InputSchema {
    pub id: Option<String>,
}

impl ParsableInput<FindTodoInput, ParseError> for InputSchema {
    fn parse(self) -> Result<FindTodoInput, ParseError> {
        let id = self
            .id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyId)?;

        Id::parse_str(&id)
            .map_err(|_| ParseError::InvalidId)
            .map(FindTodoInput::new)
    }
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    Validation(ParseError),
    NotFound,
    Internal,
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::ParsableInput;

    #[test]
    fn parse_success() {
        let input_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
        };

        assert!(input_schema.parse().is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id_schema = super::InputSchema { id: None };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err());
        assert_eq!(none_id_input.unwrap_err(), super::ParseError::EmptyId);

        let invalid_id_schema = super::InputSchema {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err());
        assert_eq!(invalid_id_input.unwrap_err(), super::ParseError::InvalidId);
    }
}
