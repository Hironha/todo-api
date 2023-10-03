use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput, CreateTodoOutput};
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};

pub async fn create_todo<T: Create>(
    ctx: CreateContext<T>,
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
pub struct CreateContext<T: Create> {
    pub store: T,
}
