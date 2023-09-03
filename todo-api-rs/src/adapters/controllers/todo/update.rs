use crate::adapters::dtos::todo::update::{Input, Output};
use crate::application::functions::todo::{
    update_todo, Update, UpdateContext, UpdateError, UpdatePayload,
};
use crate::domain::todo::Todo;

#[derive(Debug)]
pub enum RunError {
    NotFound,
    Internal,
}

pub struct UpdateController<S: Update> {
    store: S,
}

impl<S: Update> UpdateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S: Update> UpdateController<S> {
    pub async fn run(self, input: Input) -> Result<Output, RunError> {
        let context = UpdateContext { store: self.store };
        update_todo(&context, input.into_payload())
            .await
            .map(Output::from_todo)
            .map_err(|e| e.run_error())
    }
}

impl Input {
    fn into_payload(self) -> UpdatePayload {
        UpdatePayload {
            id: self.id,
            title: self.title,
            description: self.description,
            todo_at: self.todo_at,
        }
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
            updated_at: todo.updated_at.rfc3339(),
        }
    }
}

impl UpdateError {
    fn run_error(&self) -> RunError {
        match self {
            Self::NotFound => RunError::NotFound,
            Self::InternalError => RunError::Internal,
        }
    }
}
