use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput, CreateTodoOutput};
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::domain::types::DateTime;

pub async fn create_todo<S: Create>(
    ctx: CreateTodoContext<'_, S>,
    input: CreateTodoInput,
) -> CreateTodoOutput {
    let current_dt = DateTime::new();
    let payload = CreatePayload {
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
        done: false,
        created_at: current_dt,
        updated_at: current_dt,
    };

    match ctx.repository.create(payload).await {
        Ok(todo) => CreateTodoOutput::ok(todo),
        Err(err) => CreateTodoOutput::err(match err {
            CreateError::Internal => CreateTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct CreateTodoContext<'a, S: Create> {
    repository: &'a S,
}

impl<'a, S: Create> CreateTodoContext<'a, S> {
    pub const fn new(repository: &'a S) -> Self {
        Self { repository }
    }
}
