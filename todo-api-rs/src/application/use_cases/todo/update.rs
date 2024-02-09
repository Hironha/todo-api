use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::application::repositories::todo::{FindError, TodoRepository, UpdateError};
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::DateTime;

#[derive(Debug)]
pub struct UpdateTodoUseCase<T> {
    repository: T,
}

impl<T: TodoRepository> UpdateTodoUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn exec(&mut self, input: UpdateTodoInput) -> Result<(), UpdateTodoError> {
        let todo_entity = self
            .repository
            .find(input.id)
            .await
            .map_err(|err| match err {
                FindError::NotFound => UpdateTodoError::NotFound,
                FindError::Internal(err) => UpdateTodoError::Internal(err),
            })?;

        let updated_todo_entity = TodoEntity {
            title: input.title.clone(),
            description: input.description,
            todo_at: input.todo_at,
            status: input.status,
            updated_at: DateTime::now(),
            ..todo_entity
        };

        self.repository
            .update(updated_todo_entity)
            .await
            .map_err(|err| match err {
                UpdateError::NotFound => UpdateTodoError::NotFound,
                UpdateError::DuplicatedTitle => UpdateTodoError::DuplicatedTitle(input.title),
                UpdateError::Internal(err) => UpdateTodoError::Internal(err),
            })
    }
}
