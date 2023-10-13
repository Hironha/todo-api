use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput, UpdateTodoOutput};
use crate::application::repositories::todo::update::{Update, UpdateError, UpdatePayload};

pub async fn update_todo<T: Update>(
    ctx: UpdateTodoContext<T>,
    input: UpdateTodoInput,
) -> UpdateTodoOutput {
    let payload = UpdatePayload {
        id: input.id,
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
        done: input.done,
    };

    match ctx.store.update(payload).await {
        Ok(todo) => UpdateTodoOutput::ok(todo),
        Err(err) => UpdateTodoOutput::err(match err {
            UpdateError::NotFound => UpdateTodoError::NotFound,
            UpdateError::Internal => UpdateTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct UpdateTodoContext<T: Update> {
    pub store: T,
}
