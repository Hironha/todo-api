use crate::adapters::dtos::todo::create::{Input, Output, ParseError};
use crate::adapters::dtos::ParsableInput;
use crate::application::functions::todo::{
    create_todo, Create, CreateContext, CreateError, CreatePayload,
};

#[derive(Debug)]
pub enum RunError {
    Validation(ParseError),
    Internal,
}

pub struct CreateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Create,
{
    store: S,
    input: I,
}

impl<I, S> CreateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Create,
{
    pub const fn new(input: I, store: S) -> Self {
        Self { input, store }
    }

    pub async fn run(self) -> Result<Output, RunError> {
        let input = self.input.parse().map_err(RunError::Validation)?;
        let payload = CreatePayload {
            title: input.title,
            description: input.description,
            todo_at: input.todo_at,
        };

        let ctx = CreateContext { store: self.store };
        let todo = create_todo(ctx, payload).await.map_err(|err| match err {
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
