use crate::application::dtos::todo::find::FindTodoError;
use crate::application::repositories::todo::{FindError, TodoRepository};
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::Id;

#[derive(Debug)]
pub struct FindTodoUseCase<T: TodoRepository> {
    todo_repository: T,
}

impl<T: TodoRepository> FindTodoUseCase<T> {
    pub fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn exec(&self, todo_id: Id) -> Result<TodoEntity, FindTodoError> {
        self.todo_repository
            .find(todo_id)
            .await
            .map_err(|err| match err {
                FindError::NotFound => FindTodoError::NotFound,
                FindError::Internal(err) => FindTodoError::Repository(err),
            })
    }
}
