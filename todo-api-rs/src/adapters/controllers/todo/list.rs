use crate::adapters::dtos::todo::list::{Item, Output};
use crate::application::functions::todo::{list_todo, List, ListContext, ListError};
use crate::domain::todo::Todo;

#[derive(Debug)]
pub enum RunError {
    Internal,
}

pub struct ListController<S: List> {
    store: S,
}

impl<S: List> ListController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S: List> ListController<S> {
    pub async fn run(self) -> Result<Output, RunError> {
        let context = ListContext { store: self.store };
        list_todo(context)
            .await
            .map(Output::from_todos)
            .map_err(RunError::from_list)
    }
}

impl Item {
    fn from_todo(todo: Todo) -> Self {
        Self {
            id: todo.id.as_string(),
            title: todo.title,
            description: todo.description,
            todo_at: todo.todo_at.map(|at| at.ymd()),
            created_at: todo.created_at.rfc3339(),
            updated_at: todo.updated_at.rfc3339(),
        }
    }
}

impl Output {
    fn from_todos(todos: Vec<Todo>) -> Self {
        Self {
            count: todos.len(),
            items: todos.into_iter().map(Item::from_todo).collect(),
        }
    }
}

impl RunError {
    fn from_list(error: ListError) -> Self {
        match error {
            ListError::StorageAccess => Self::Internal,
        }
    }
}
