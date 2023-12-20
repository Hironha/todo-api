use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
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
            .unwrap_or_default()
            .into_iter()
            .map(|id| Id::parse_str(&id).or(Err(ParseError::InvalidTag(id))))
            .collect::<Result<Vec<Id>, ParseError>>()?;

        Ok(BindTodoTagsInput { todo_id, tags_id })
    }
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    Parsing(ParseError),
    #[error(transparent)]
    Binding(BindTodoTagsError),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("todo id is required")]
    EmptyTodo,
    #[error("invalid todo id format")]
    InvalidTodo,
    #[error("invalid tag id format")]
    InvalidTag(String),
}
