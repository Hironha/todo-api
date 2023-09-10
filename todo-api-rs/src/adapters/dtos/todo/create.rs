use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::domain::types::Date;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Title,
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Title => "required",
            Self::TodoAt => "optional, but if defined, should be an UTC date on YYYY-MM-DD format",
        };
        description.into()
    }
}

#[derive(Debug)]
pub struct Input {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

#[derive(Debug)]
pub struct InputSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl ParsableInput<Input, ParseError> for InputSchema {
    fn parse(self) -> Result<Input, ParseError> {
        let title = self
            .title
            .filter(|t| !t.is_empty())
            .ok_or(ParseError::Title)?;

        let description = self.description.filter(|d| !d.is_empty());

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(Input {
            title,
            description,
            todo_at,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    Validation(ParseError),
    Internal,
}

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

#[cfg(test)]
mod tests {
    use crate::adapters::dtos::ParsableInput;

    #[test]
    fn parse_success() {
        let input_schema = super::InputSchema {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-11".to_string()),
        };

        assert!(input_schema.parse().is_ok());
    }

    #[test]
    fn parse_empty_description_to_none() {
        let input_schema = super::InputSchema {
            title: Some("title".to_string()),
            description: Some("".to_string()),
            todo_at: None,
        };

        assert!(input_schema.parse().is_ok_and(|p| p.description.is_none()));
    }

    #[test]
    fn parse_title_fail() {
        let none_title_schema = super::InputSchema {
            title: None,
            description: Some("description".to_string()),
            todo_at: None,
        };
        let none_tile_input = none_title_schema.parse();

        assert!(none_tile_input.is_err_and(|e| e == super::ParseError::Title));

        let empty_title_schema = super::InputSchema {
            title: Some("".to_string()),
            description: None,
            todo_at: None,
        };
        let empty_title_input = empty_title_schema.parse();

        assert!(empty_title_input.is_err_and(|e| e == super::ParseError::Title));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at_schema = super::InputSchema {
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("2023-2023-2023".to_string()),
        };

        let invalid_todo_at_input = invalid_todo_at_schema.parse();

        assert!(invalid_todo_at_input.is_err_and(|e| e == super::ParseError::TodoAt));
    }
}
