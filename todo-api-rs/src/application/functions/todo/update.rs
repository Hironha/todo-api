use crate::application::repositories::todo::update::{Update, UpdateError, UpdatePayload};
use crate::domain::todo::{Description, Title, Todo};
use crate::domain::types::{Date, Id};

pub async fn update_todo<T: Update>(
    ctx: UpdateTodoContext<T>,
    input: UpdateTodoInput,
) -> Result<Todo, UpdateTodoError> {
    let payload = UpdatePayload {
        id: input.id,
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
    };
    ctx.store.set(payload).await.map_err(|e| match e {
        UpdateError::NotFound => UpdateTodoError::NotFound,
        UpdateError::Internal => UpdateTodoError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct UpdateTodoInput {
    pub id: Id,
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

pub struct UpdateTodoContext<T: Update> {
    pub store: T,
}

#[derive(Debug)]
pub enum UpdateTodoError {
    NotFound,
    Internal,
}
