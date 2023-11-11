use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

use serde::Serialize;

use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::list::{ListTodoError, ListTodoInput};
use crate::domain::entities::todo::{Title, TitleError};

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub title: Option<String>,
}

impl Parse<ListTodoInput, ParseError> for ListRequest {
    fn parse(self) -> Result<ListTodoInput, ParseError> {
        let page = NonZeroU32::new(self.page.unwrap_or(1)).ok_or(ParseError::InvalidPage)?;
        let per_page =
            NonZeroU32::new(self.per_page.unwrap_or(10)).ok_or(ParseError::InvalidPerPage)?;

        let title = self
            .title
            .filter(|t| !t.is_empty())
            .map(Title::new)
            .transpose()
            .map_err(ParseError::Title)?;

        Ok(ListTodoInput {
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

#[derive(Debug)]
pub enum RunError {
    Parsing(ParseError),
    Listing(ListTodoError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(_) => write!(f, "failed parsing list input"),
            Self::Listing(_) => write!(f, "failed listing todo"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Listing(err) => Some(err),
            Self::Parsing(err) => Some(err),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidPage,
    InvalidPerPage,
    Title(TitleError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPage | Self::InvalidPerPage => {
                write!(f, "optional non zero natural number")
            }
            Self::Title(err) => write!(f, "optional {err}"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPage | Self::InvalidPerPage => None,
            Self::Title(err) => Some(err),
        }
    }
}
