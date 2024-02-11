use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput, UpdateTodoOutput};
use crate::application::repositories::todo::{TodoRepository, UpdateError, UpdateQuery};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct UpdateTodoUseCase<T> {
    repository: T,
}

impl<T: TodoRepository> UpdateTodoUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TodoRepository> UseCase<UpdateTodoInput, UpdateTodoOutput> for UpdateTodoUseCase<T> {
    async fn exec(mut self, input: UpdateTodoInput) -> UpdateTodoOutput {
        let query = UpdateQuery {
            id: input.id,
            title: input.title.clone(),
            description: input.description,
            status: input.status,
            todo_at: input.todo_at,
        };

        self.repository
            .update(query)
            .await
            .map_err(|err| match err {
                UpdateError::NotFound => UpdateTodoError::NotFound,
                UpdateError::DuplicatedTitle => UpdateTodoError::DuplicatedTitle(input.title),
                UpdateError::Internal(err) => UpdateTodoError::Internal(err),
            })
    }
}
