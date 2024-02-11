use std::error::Error;

use serde::Serialize;

use super::TodoView;

use crate::adapters::dtos::todo::create::CreateTodoPresenter;
use crate::adapters::dtos::todo::delete::DeleteTodoPresenter;
use crate::adapters::dtos::todo::find::FindTodoPresenter;
use crate::adapters::dtos::todo::list::ListTodosPresenter;
use crate::adapters::dtos::todo::update::UpdateTodoPresenter;
use crate::application::dtos::todo::list::TodosList;
use crate::domain::entities::todo::TodoEntity;

#[derive(Clone, Debug, Serialize)]
pub struct ListTodosView {
    pub page: u32,
    #[serde(rename(serialize = "perPage"))]
    pub per_page: u32,
    pub count: u64,
    pub items: Vec<TodoView>,
}

impl From<TodosList> for ListTodosView {
    fn from(value: TodosList) -> Self {
        Self {
            page: value.page.into(),
            per_page: value.per_page.into(),
            count: value.count,
            items: value.items.into_iter().map(TodoView::from).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct JsonTodoPresenter;

impl JsonTodoPresenter {
    pub const fn new() -> Self {
        Self
    }
}

impl CreateTodoPresenter for JsonTodoPresenter {
    type View = Result<TodoView, Box<dyn Error>>;

    fn present(&self, result: Result<TodoEntity, Box<dyn Error>>) -> Self::View {
        result.map(TodoView::from)
    }
}

impl DeleteTodoPresenter for JsonTodoPresenter {
    type View = Result<(), Box<dyn Error>>;

    fn present(&self, result: Result<(), Box<dyn Error>>) -> Self::View {
        result
    }
}

impl FindTodoPresenter for JsonTodoPresenter {
    type View = Result<TodoView, Box<dyn Error>>;

    fn present(&self, result: Result<TodoEntity, Box<dyn Error>>) -> Self::View {
        result.map(TodoView::from)
    }
}

impl ListTodosPresenter for JsonTodoPresenter {
    type View = Result<ListTodosView, Box<dyn Error>>;

    fn present(&self, result: Result<TodosList, Box<dyn Error>>) -> Self::View {
        result.map(ListTodosView::from)
    }
}

impl UpdateTodoPresenter for JsonTodoPresenter {
    type View = Result<(), Box<dyn Error>>;

    fn present(&self, result: Result<(), Box<dyn Error>>) -> Self::View {
        result
    }
}
