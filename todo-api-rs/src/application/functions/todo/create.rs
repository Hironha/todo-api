use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::domain::entities::todo::{Description, Title, Todo};
use crate::domain::types::Date;

pub async fn create_todo<T: Create>(
    ctx: CreateContext<T>,
    input: CreateTodoInput,
) -> Result<Todo, CreateTodoError> {
    let payload = CreatePayload {
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
    };

    ctx.store.create(payload).await.map_err(|e| match e {
        CreateError::Internal => CreateTodoError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

pub struct CreateContext<T: Create> {
    pub store: T,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateTodoError {
    Internal,
}
