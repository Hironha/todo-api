use crate::application::dtos::todo::delete::DeleteTodoError;
use crate::application::repositories::todo::{DeleteError, TodoRepository};
use crate::domain::types::Id;

#[derive(Debug)]
pub struct DeleteTodoUseCase<T> {
    todo_repository: T,
}

impl<T: TodoRepository> DeleteTodoUseCase<T> {
    pub fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn exec(&mut self, todo_id: Id) -> Result<(), DeleteTodoError> {
        self.todo_repository
            .delete(todo_id)
            .await
            .map_err(|err| match err {
                DeleteError::NotFound => DeleteTodoError::NotFound,
                DeleteError::Internal(err) => DeleteTodoError::Internal(err),
            })
    }
}
