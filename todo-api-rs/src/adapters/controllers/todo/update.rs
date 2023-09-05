use crate::adapters::dtos::todo::update::{Input, Output};
use crate::application::functions::todo::{
    update_todo, Update, UpdateContext, UpdateError, UpdatePayload,
};

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
        let ctx = UpdateContext { store: self.store };
        let payload = UpdatePayload {
            id: input.id,
            title: input.title,
            description: input.description,
            todo_at: input.todo_at,
        };

        let todo = update_todo(&ctx, payload).await.map_err(|err| match err {
            UpdateError::NotFound => RunError::NotFound,
            UpdateError::InternalError => RunError::Internal,
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
