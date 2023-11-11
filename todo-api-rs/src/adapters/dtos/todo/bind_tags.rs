use std::error::Error;
use std::fmt;

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
            .map(|ids| {
                ids.into_iter()
                    .map(|id| Id::parse_str(&id).or(Err(ParseError::InvalidTag(id))))
                    .collect::<Result<Vec<Id>, ParseError>>()
            })
            .transpose()?;

        Ok(BindTodoTagsInput { todo_id, tags_id })
    }
}

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Binding(BindTodoTagsError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing bind input"),
            Self::Binding(_) => write!(f, "failed binding todo tags"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parsing(err) => Some(err),
            Self::Binding(err) => Some(err),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyTodo,
    InvalidTodo,
    InvalidTag(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTodo => write!(f, "required string"),
            Self::InvalidTodo => write!(f, "invalid id format"),
            Self::InvalidTag(id) => write!(f, "invalid id {id} format"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyTodo | Self::InvalidTag(_) | Self::InvalidTodo => None,
        }
    }
}
