use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::application::repositories::todo::{FindError, TodoRepository, UpdateError};

pub async fn update_todo<T: TodoRepository>(
    ctx: UpdateTodoContext<'_, T>,
    input: UpdateTodoInput,
) -> Result<(), UpdateTodoError> {
    let mut todo_entity = ctx
        .todo_repository
        .find(input.id)
        .await
        .map_err(|err| match err {
            FindError::NotFound => UpdateTodoError::NotFound,
            FindError::Internal(err) => UpdateTodoError::Repository(err),
        })?;

    todo_entity.title = input.title;
    todo_entity.description = input.description;
    todo_entity.todo_at = input.todo_at;
    todo_entity.status = input.status;

    ctx.todo_repository
        .update(todo_entity)
        .await
        .map_err(|err| match err {
            UpdateError::NotFound => UpdateTodoError::NotFound,
            UpdateError::Internal(err) => UpdateTodoError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct UpdateTodoContext<'a, T>
where
    T: TodoRepository,
{
    pub todo_repository: &'a T,
}
