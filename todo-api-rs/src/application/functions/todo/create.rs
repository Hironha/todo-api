use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput, CreateTodoOutput};
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};

pub async fn create_todo<S: Create>(
    ctx: CreateContext<'_, S>,
    input: CreateTodoInput,
) -> CreateTodoOutput {
    let payload = CreatePayload {
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
    };

    match ctx.store.create(payload).await {
        Ok(todo) => CreateTodoOutput::ok(todo),
        Err(err) => CreateTodoOutput::err(match err {
            CreateError::Internal => CreateTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct CreateContext<'a, S: Create> {
    store: &'a S,
}

impl<'a, S: Create> CreateContext<'a, S> {
    pub const fn new(store: &'a S) -> Self {
        Self { store }
    }
}
