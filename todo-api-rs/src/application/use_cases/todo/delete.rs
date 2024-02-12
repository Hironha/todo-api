use crate::application::dtos::todo::delete::{DeleteTodoError, DeleteTodoInput, DeleteTodoOutput};
use crate::application::repositories::todo::{DeleteError, TodoRepository};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct DeleteTodoUseCase<T> {
    repository: T,
}

impl<T: TodoRepository> DeleteTodoUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TodoRepository> UseCase<DeleteTodoInput, DeleteTodoOutput> for DeleteTodoUseCase<T> {
    async fn exec(mut self, todo_id: DeleteTodoInput) -> DeleteTodoOutput {
        self.repository
            .delete(todo_id)
            .await
            .map_err(|err| match err {
                DeleteError::NotFound => DeleteTodoError::NotFound,
                DeleteError::Internal(err) => DeleteTodoError::Internal(err),
            })
    }
}
