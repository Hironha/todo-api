use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::domain::types::{Date, Id};

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
pub struct Input {
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

#[derive(Debug)]
pub struct InputSchema {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl ParsableInput<Input, ParseError> for InputSchema {
    fn parse(self) -> Result<Input, ParseError> {
        let id = parse_id(self.id)?;
        let title = parse_title(self.title)?;
        let description = parse_description(self.description)?;
        let todo_at = parse_todo_at(self.todo_at)?;

        Ok(Input {
            id,
            title,
            description,
            todo_at,
        })
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
    Id,
    Title,
    TitleLength,
    DescriptionLength,
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Id => "required string",
            Self::Title => "required string",
            Self::TitleLength => "maximum of 64 characters",
            Self::DescriptionLength => "maximum of 256 characters",
            Self::TodoAt => "optional string, but if defined, should be a date on Y-M-D format",
        };
        description.to_string()
    }
}

fn parse_id(id: Option<String>) -> Result<Id, ParseError> {
    let Some(id) = id else {
        return Err(ParseError::Id);
    };
    Id::parse_str(&id).map_err(|_| ParseError::Id)
}

fn parse_title(title: Option<String>) -> Result<String, ParseError> {
    let title = title.filter(|t| !t.is_empty()).ok_or(ParseError::Title)?;
    if title.len() > 64 {
        return Err(ParseError::TitleLength);
    }

    Ok(title)
}

fn parse_description(description: Option<String>) -> Result<Option<String>, ParseError> {
    match description.filter(|d| !d.is_empty()) {
        Some(d) if d.len() <= 256 => Ok(Some(d)),
        Some(_) => Err(ParseError::DescriptionLength),
        None => Ok(None),
    }
}

fn parse_todo_at(todo_at: Option<String>) -> Result<Option<Date>, ParseError> {
    let Some(todo_at) = todo_at else {
        return Ok(None);
    };

    Date::parse_str(&todo_at)
        .map(Some)
        .map_err(|_| ParseError::TodoAt)
}

#[cfg(test)]
mod tests {
    use super::ParsableInput;

    #[test]
    fn parse_success() {
        let input_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
        };

        assert!(input_schema.parse().is_ok());
    }

    #[test]
    fn parse_id_fail() {
        let none_id_schema = super::InputSchema {
            id: None,
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
        };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err_and(|e| e == super::ParseError::Id));

        let invalid_id_schema = super::InputSchema {
            id: Some("invalid-id".to_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: None,
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err_and(|e| e == super::ParseError::Id));
    }

    #[test]
    fn parse_title_fail() {
        let none_title_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
            title: None,
            description: None,
            todo_at: None,
        };
        let none_title_input = none_title_schema.parse();

        assert!(none_title_input.is_err_and(|e| e == super::ParseError::Title));

        let empty_title_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
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
            id: Some(super::Id::new().as_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("todo_at".to_string()),
        };
        let invalid_todo_at_input = invalid_todo_at_schema.parse();

        assert!(invalid_todo_at_input.is_err_and(|e| e == super::ParseError::TodoAt))
    }
}
