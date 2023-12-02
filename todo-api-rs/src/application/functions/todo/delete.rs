use crate::application::dtos::todo::delete::{DeleteTodoError, DeleteTodoInput};
use crate::application::repositories::todo::{DeleteError, TodoRepository};

pub async fn delete_todo<T>(
    ctx: DeleteTodoContext<'_, T>,
    DeleteTodoInput(id): DeleteTodoInput,
) -> Result<(), DeleteTodoError>
where
    T: TodoRepository,
{
    ctx.todo_repository.delete(id).await.map_err(|err| match err {
        DeleteError::NotFound => DeleteTodoError::NotFound,
        DeleteError::Internal(err) => DeleteTodoError::Repository(err),
    })
}

#[derive(Clone, Debug)]
pub struct DeleteTodoContext<'a, T>
where
    T: TodoRepository,
{
    pub todo_repository: &'a T,
}
