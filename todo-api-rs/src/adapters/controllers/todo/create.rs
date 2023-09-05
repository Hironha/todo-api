use crate::adapters::dtos::todo::create::{Input, Output};
use crate::application::functions::todo::{
    create_todo, Create, CreateContext, CreateError, CreatePayload,
};

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
        let payload = CreatePayload {
            title: input.title,
            description: input.description,
            todo_at: input.todo_at,
        };

        let todo = create_todo(&context, payload)
            .await
            .map_err(|err| match err {
                CreateError::InternalError => RunError::Internal,
            })?;

        Ok(Output {
            id: todo.id.as_string(),
            title: todo.title,
            description: todo.description,
            todo_at: todo.todo_at.map(|at| at.ymd()),
            created_at: todo.created_at.rfc3339(),
            updated_at: todo.updated_at.rfc3339(),
        })
    }
}
