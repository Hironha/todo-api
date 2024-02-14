use std::error;
use std::num::NonZeroU32;

use thiserror::Error;

use crate::application::dtos::todo::list::{ListTodosInput, TodosList};
use crate::domain::entities::todo::{Title, TitleError};

pub trait ListPresenter {
    type View;
    fn present(&self, response: ListResponse) -> Self::View;
}

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub title: Option<String>,
}

impl ListRequest {
    pub fn parse(self) -> Result<ListTodosInput, ParseError> {
        let page = NonZeroU32::new(self.page.unwrap_or(1)).ok_or(ParseError::Page)?;
        let per_page = NonZeroU32::new(self.per_page.unwrap_or(10)).ok_or(ParseError::PerPage)?;

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

pub type ListResponse = Result<TodosList, ListResponseError>;

#[derive(Debug, Error)]
pub enum ListResponseError {
    #[error(transparent)]
    Input(ParseError),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Page should be an integer ranging from 0 to {}", u32::MAX)]
    Page,
    #[error("Per page should be an integer ranging from 0 to {}", u32::MAX)]
    PerPage,
    #[error(transparent)]
    Title(TitleError),
}
