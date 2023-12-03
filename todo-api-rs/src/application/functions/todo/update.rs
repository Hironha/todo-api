use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::application::repositories::todo::{FindError, TodoRepository, UpdateError};
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::DateTime;

pub async fn update_todo<T: TodoRepository>(
    ctx: UpdateTodoContext<'_, T>,
    input: UpdateTodoInput,
) -> Result<(), UpdateTodoError> {
    let todo_entity = ctx
        .todo_repository
        .find(input.id)
        .await
        .map_err(|err| match err {
            FindError::NotFound => UpdateTodoError::NotFound,
            FindError::Internal(err) => UpdateTodoError::Repository(err),
        })?;

    let updated_todo_entity = TodoEntity {
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
        status: input.status,
        updated_at: DateTime::new(),
        ..todo_entity
    };

    ctx.todo_repository
        .update(updated_todo_entity)
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
