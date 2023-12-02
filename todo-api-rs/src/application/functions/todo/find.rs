use crate::application::dtos::todo::find::{FindTodoError, FindTodoInput};
use crate::application::repositories::todo::{FindError, TodoRepository};
use crate::domain::entities::todo::TodoEntity;

pub async fn find_todo<T>(
    ctx: FindTodoContext<'_, T>,
    FindTodoInput(id): FindTodoInput,
) -> Result<TodoEntity, FindTodoError>
where
    T: TodoRepository,
{
    ctx.todo_repository.find(id).await.map_err(|err| match err {
        FindError::NotFound => FindTodoError::NotFound,
        FindError::Internal(err) => FindTodoError::Repository(err),
    })
}

#[derive(Clone, Debug)]
pub struct FindTodoContext<'a, T>
where
    T: TodoRepository,
{
    pub todo_repository: &'a T,
}
