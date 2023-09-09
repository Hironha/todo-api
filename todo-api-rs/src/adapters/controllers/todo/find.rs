use crate::adapters::dtos::todo::find::{Input, Output, ParseError};
use crate::adapters::dtos::ParsableInput;
use crate::application::functions::todo::{find_todo, Find, FindContext, FindError, FindPayload};

#[derive(Debug)]
pub enum RunError {
    Validation(ParseError),
    NotFound,
    Internal,
}

pub struct FindController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Find,
{
    input: I,
    store: S,
}

impl<I, S> FindController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Find,
{
    pub const fn new(input: I, store: S) -> Self {
        Self { input, store }
    }

    pub async fn run(self) -> Result<Output, RunError> {
        let input = self.input.parse().map_err(RunError::Validation)?;
        let payload = FindPayload { id: input.id };

        let ctx = FindContext { store: self.store };
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
