use serde::Serialize;
use std::num::NonZeroU32;

use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
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
            items: list.items.into_iter().map(TodoView::from).collect(),
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
    pub items: Vec<TodoView>,
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
}

impl ParsableInput<ListTodoInput, ParseError> for RawInput {
    fn parse(self) -> Result<ListTodoInput, ParseError> {
        let page = NonZeroU32::new(self.page.unwrap_or(1u32)).ok_or(ParseError::InvalidPage)?;
        let per_page =
            NonZeroU32::new(self.per_page.unwrap_or(10u32)).ok_or(ParseError::InvalidPerPage)?;

        Ok(ListTodoInput { page, per_page })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidPage,
    InvalidPerPage,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::InvalidPage | Self::InvalidPerPage => {
                "optional non zero positive interger".into()
            }
        }
    }
}
