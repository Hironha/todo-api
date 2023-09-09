use crate::adapters::dtos::todo::update::{Input, Output, ParseError};
use crate::adapters::dtos::ParsableInput;
use crate::application::functions::todo::{
    update_todo, Update, UpdateContext, UpdateError, UpdatePayload,
};

#[derive(Debug)]
pub enum RunError {
    Validation(ParseError),
    NotFound,
    Internal,
}

pub struct UpdateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Update,
{
    input: I,
    store: S,
}

impl<I, S> UpdateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Update,
{
    pub const fn new(input: I, store: S) -> Self {
        Self { input, store }
    }

    pub async fn run(self) -> Result<Output, RunError> {
        let input = self.input.parse().map_err(RunError::Validation)?;
        let payload = UpdatePayload {
            id: input.id,
            title: input.title,
            description: input.description,
            todo_at: input.todo_at,
        };

        let ctx = UpdateContext { store: self.store };
        let todo = update_todo(ctx, payload).await.map_err(|err| match err {
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
