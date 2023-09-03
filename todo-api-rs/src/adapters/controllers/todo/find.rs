use crate::adapters::dtos::todo::find::{Input, Output};
use crate::application::functions::todo::{find_todo, Find, FindContext, FindError, FindPayload};
use crate::domain::todo::Todo;

#[derive(Debug)]
pub enum RunError {
    NotFound,
    Internal,
}

pub struct FindController<S: Find> {
    store: S,
}

impl<S: Find> FindController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S: Find> FindController<S> {
    pub async fn run(self, input: Input) -> Result<Output, RunError> {
        let context = FindContext { store: self.store };
        find_todo(context, input.into_payload())
            .await
            .map(Output::from_todo)
            .map_err(|e| e.run_error())
    }
}

impl Input {
    fn into_payload(self) -> FindPayload {
        FindPayload { id: self.id }
    }
}

impl Output {
    fn from_todo(todo: Todo) -> Self {
        Self {
            id: todo.id.as_string(),
            title: todo.title,
            description: todo.description,
            todo_at: todo.todo_at.map(|at| at.ymd()),
            created_at: todo.created_at.rfc3339(),
            updated_at: todo.created_at.rfc3339(),
        }
    }
}

impl FindError {
    fn run_error(&self) -> RunError {
        match self {
            Self::InternalError => RunError::Internal,
            Self::NotFound => RunError::NotFound,
        }
    }
}
