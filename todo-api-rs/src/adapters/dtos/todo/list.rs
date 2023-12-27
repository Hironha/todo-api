use std::num::NonZeroU32;

use serde::Serialize;
use thiserror::Error;

use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::list::{ListTodoError, ListTodosInput};
use crate::domain::entities::todo::{Title, TitleError};

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub title: Option<String>,
}

impl Parse<ListTodosInput, ParseError> for ListRequest {
    fn parse(self) -> Result<ListTodosInput, ParseError> {
        let page = NonZeroU32::new(self.page.unwrap_or(1)).ok_or(ParseError::InvalidPage)?;
        let per_page =
            NonZeroU32::new(self.per_page.unwrap_or(10)).ok_or(ParseError::InvalidPerPage)?;

        let title = self
            .title
            .filter(|t| !t.is_empty())
            .map(Title::new)
            .transpose()
            .map_err(ParseError::Title)?;

        Ok(ListTodosInput {
            page,
            per_page,
            title,
        })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ListResponse {
    pub page: u32,
    #[serde(rename(serialize = "perPage"))]
    pub per_page: u32,
    pub count: u64,
    pub items: Vec<TodoPresenter>,
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    Parsing(ParseError),
    #[error(transparent)]
    Listing(ListTodoError),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("invalid page: should be an integer ranging from 0 to {}", u32::MAX)]
    InvalidPage,
    #[error(
        "invalid per page: should be an integer ranging from 0 to {}",
        u32::MAX
    )]
    InvalidPerPage,
    #[error("invalid title: {0}")]
    Title(TitleError),
}
