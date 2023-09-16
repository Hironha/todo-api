use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
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
pub struct Input {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

#[derive(Debug)]
pub struct RawInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl ParsableInput<Input, ParseError> for RawInput {
    fn parse(self) -> Result<Input, ParseError> {
        let title = parse_title(self.title)?;
        let description = parse_description(self.description)?;
        let todo_at = parse_todo_at(self.todo_at)?;

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

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Title,
    TitleLength,
    DescriptionLength,
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Title => "required string",
            Self::TitleLength => "maximum of 64 characters",
            Self::DescriptionLength => "maximum of 256 characters",
            Self::TodoAt => {
                "optional string, but, if defined, should be an UTC date on YYYY-MM-DD format"
            }
        };
        description.into()
    }
}

fn parse_title(title: Option<String>) -> Result<String, ParseError> {
    let title = title.filter(|t| !t.is_empty()).ok_or(ParseError::Title)?;
    if title.len() <= 64 {
        Ok(title)
    } else {
        Err(ParseError::TitleLength)
    }
}

fn parse_description(description: Option<String>) -> Result<Option<String>, ParseError> {
    match description.filter(|d| !d.is_empty()) {
        Some(d) if d.len() <= 256 => Ok(Some(d)),
        Some(_) => Err(ParseError::DescriptionLength),
        None => Ok(None),
    }
}

fn parse_todo_at(todo_at: Option<String>) -> Result<Option<Date>, ParseError> {
    todo_at
        .map(|at| Date::parse_str(&at))
        .transpose()
        .map_err(|_| ParseError::TodoAt)
}
