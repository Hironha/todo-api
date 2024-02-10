use crate::application::dtos::todo::find::{FindTodoError, FindTodoInput, FindTodoOutput};
use crate::application::repositories::todo::{FindError, TodoRepository};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct FindTodoUseCase<T> {
    repository: T,
}

impl<T: TodoRepository> FindTodoUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TodoRepository> UseCase<FindTodoInput, FindTodoOutput> for FindTodoUseCase<T> {
    async fn exec(self, input: FindTodoInput) -> FindTodoOutput {
        self.repository.find(input).await.map_err(|err| match err {
            FindError::NotFound => FindTodoError::NotFound,
            FindError::Internal(err) => FindTodoError::Internal(err),
        })
    }
}
