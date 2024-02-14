use serde::Serialize;

use super::error::{Content, JsonError};
use super::TodoView;

use crate::adapters::dtos::todo::create::{CreatePresenter, CreateResponse, CreateResponseError};
use crate::adapters::dtos::todo::delete::{DeletePresenter, DeleteResponse, DeleteResponseError};
use crate::adapters::dtos::todo::find::{FindPresenter, FindResponse, FindResponseError};
use crate::adapters::dtos::todo::list::{ListPresenter, ListResponse, ListResponseError};
use crate::adapters::dtos::todo::update::{UpdatePresenter, UpdateResponse, UpdateResponseError};

#[derive(Clone, Debug, Serialize)]
pub struct TodosListView {
    pub page: u32,
    #[serde(rename(serialize = "perPage"))]
    pub per_page: u32,
    pub count: u64,
    pub items: Vec<TodoView>,
}

#[derive(Clone, Debug)]
pub struct JsonTodoPresenter;

impl JsonTodoPresenter {
    pub const fn new() -> Self {
        Self
    }
}

impl CreatePresenter for JsonTodoPresenter {
    type View = Result<TodoView, JsonError>;

    fn present(&self, response: CreateResponse) -> Self::View {
        response.map(TodoView::from).map_err(|err| match err {
            CreateResponseError::Input(parse_err) => {
                let content = Content::new("ParseError", parse_err.to_string());
                JsonError::new(400, content)
            }
            CreateResponseError::DuplicatedTitle(..) => {
                let content = Content::new("DuplicatedTitle", err.to_string());
                JsonError::new(409, content)
            }
            CreateResponseError::Internal(src) => JsonError::internal().with_src(src),
        })
    }
}

impl DeletePresenter for JsonTodoPresenter {
    type View = Result<(), JsonError>;

    fn present(&self, response: DeleteResponse) -> Self::View {
        response.map_err(|err| match err {
            DeleteResponseError::Input(parse_err) => {
                let content = Content::new("ParseError", parse_err.to_string());
                JsonError::new(400, content)
            }
            DeleteResponseError::NotFound(..) => {
                let content = Content::new("NotFound", err.to_string());
                JsonError::new(404, content)
            }
            DeleteResponseError::Internal(src) => JsonError::internal().with_src(src),
        })
    }
}

impl FindPresenter for JsonTodoPresenter {
    type View = Result<TodoView, JsonError>;

    fn present(&self, response: FindResponse) -> Self::View {
        response.map(TodoView::from).map_err(|err| match err {
            FindResponseError::Input(parse_err) => {
                let content = Content::new("ParseError", parse_err.to_string());
                JsonError::new(400, content)
            }
            FindResponseError::NotFound(..) => {
                let content = Content::new("NotFound", err.to_string());
                JsonError::new(404, content)
            }
            FindResponseError::Internal(src) => JsonError::internal().with_src(src),
        })
    }
}

impl ListPresenter for JsonTodoPresenter {
    type View = Result<TodosListView, JsonError>;

    fn present(&self, response: ListResponse) -> Self::View {
        response
            .map(|list| TodosListView {
                page: list.page.into(),
                per_page: list.per_page.into(),
                count: list.count,
                items: list.items.into_iter().map(TodoView::from).collect(),
            })
            .map_err(|err| match err {
                ListResponseError::Input(parse_err) => {
                    let content = Content::new("ParseError", parse_err.to_string());
                    JsonError::new(400, content)
                }
                ListResponseError::Internal(src) => JsonError::internal().with_src(src),
            })
    }
}

impl UpdatePresenter for JsonTodoPresenter {
    type View = Result<(), JsonError>;

    fn present(&self, response: UpdateResponse) -> Self::View {
        response.map_err(|err| match err {
            UpdateResponseError::Input(parse_err) => {
                let content = Content::new("ParseError", parse_err.to_string());
                JsonError::new(400, content)
            }
            UpdateResponseError::DuplicatedTitle(..) => {
                let content = Content::new("DuplicatedTitle", err.to_string());
                JsonError::new(409, content)
            }
            UpdateResponseError::NotFound(..) => {
                let content = Content::new("NotFound", err.to_string());
                JsonError::new(404, content)
            }
            UpdateResponseError::Internal(src) => JsonError::internal().with_src(src),
        })
    }
}
