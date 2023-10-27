use serde::Serialize;
use std::num::NonZeroU32;

use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::list::{ListTodoInput, TodoList};

#[derive(Clone, Debug)]
pub struct Output(Result<OutputData, RunError>);

impl Output {
    pub const fn err(error: RunError) -> Self {
        Self(Err(error))
    }

    pub fn from_list(list: TodoList) -> Self {
        let data = OutputData {
            count: list.count,
            items: list.items.into_iter().map(TodoPresenter::from).collect(),
        };

        Self(Ok(data))
    }

    pub fn into_result(self) -> Result<OutputData, RunError> {
        self.0
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct OutputData {
    pub count: u64,
    pub items: Vec<TodoPresenter>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Parsing(ParseError),
    Internal,
}

#[derive(Clone, Debug)]
pub struct RawInput {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub title: Option<String>,
}

impl Parse<ListTodoInput, ParseError> for RawInput {
    fn parse(self) -> Result<ListTodoInput, ParseError> {
        let page = NonZeroU32::new(self.page.unwrap_or(1u32)).ok_or(ParseError::InvalidPage)?;
        let per_page =
            NonZeroU32::new(self.per_page.unwrap_or(10u32)).ok_or(ParseError::InvalidPerPage)?;

        // title cannot have more than 256 characters
        let title = self
            .title
            .filter(|t| !t.is_empty())
            .map(|t| (t.len() <= 256).then_some(t).ok_or(ParseError::TitleLength))
            .transpose()?;

        Ok(ListTodoInput {
            page,
            per_page,
            title,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidPage,
    InvalidPerPage,
    TitleLength,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::InvalidPage | Self::InvalidPerPage => "optional non zero positive integer".into(),
            Self::TitleLength => "optional string with less than 256 characters".into(),
        }
    }
}
