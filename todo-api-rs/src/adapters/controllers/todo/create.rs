use crate::adapters::dtos::todo::create::{Input, Output};
use crate::application::functions::todo::{
    create_todo, Create, CreateContext, CreateError, CreatePayload,
};
use crate::domain::todo::Todo;

#[derive(Debug)]
pub enum RunError {
    Internal,
}

pub struct CreateController<S: Create> {
    store: S,
}

impl<S: Create> CreateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S: Create> CreateController<S> {
    pub async fn run(self, input: Input) -> Result<Output, RunError> {
        let context = CreateContext { store: self.store };
        create_todo(&context, input.into_payload())
            .await
            .map(Output::from_todo)
            .map_err(|e| e.run_error())
    }
}

impl Input {
    fn into_payload(self) -> CreatePayload {
        CreatePayload {
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

impl CreateError {
    fn run_error(&self) -> RunError {
        match self {
            Self::InternalError => RunError::Internal,
        }
    }
}
