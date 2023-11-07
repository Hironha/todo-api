use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::bind_tags::BindTodoTagsInput;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct BindTagsRequest {
    pub todo_id: Option<String>,
    pub tags_id: Option<Vec<String>>,
}

impl Parse<BindTodoTagsInput, ParseError> for BindTagsRequest {
    fn parse(self) -> Result<BindTodoTagsInput, ParseError> {
        let todo_id = self
            .todo_id
            .filter(|id| !id.is_empty())
            .ok_or(ParseError::EmptyTodo)
            .and_then(|id| Id::parse_str(&id).or(Err(ParseError::InvalidTodo)))?;

        let tags_id = self
            .tags_id
            .map(|ids| {
                ids.into_iter()
                    .map(|id| Id::parse_str(&id).or(Err(ParseError::InvalidTag(id))))
                    .collect::<Result<Vec<Id>, ParseError>>()
            })
            .transpose()?;

        Ok(BindTodoTagsInput { todo_id, tags_id })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    TodoNotFound,
    TagNotFound,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyTodo,
    InvalidTodo,
    InvalidTag(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTodo => write!(f, "required string"),
            Self::InvalidTodo => write!(f, "invalid id format"),
            Self::InvalidTag(id) => write!(f, "invalid id {id} format"),
        }
    }
}
