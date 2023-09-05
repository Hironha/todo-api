use crate::adapters::dtos::todo::find::{Input, Output};
use crate::application::functions::todo::{find_todo, Find, FindContext, FindError, FindPayload};

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
        let ctx = FindContext { store: self.store };
        let payload = FindPayload { id: input.id };

        let todo = find_todo(ctx, payload).await.map_err(|err| match err {
            FindError::InternalError => RunError::Internal,
            FindError::NotFound => RunError::NotFound,
        })?;

        Ok(Output {
            id: todo.id.as_string(),
            title: todo.title,
            description: todo.description,
            todo_at: todo.todo_at.map(|at| at.ymd()),
            created_at: todo.created_at.rfc3339(),
            updated_at: todo.created_at.rfc3339(),
        })
    }
}
