use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::application::repositories::todo::{FindError, TodoRepository, UpdateError};
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::DateTime;

#[derive(Debug)]
pub struct UpdateTodoUseCase<T: TodoRepository> {
    todo_repository: T,
}

impl<T: TodoRepository> UpdateTodoUseCase<T> {
    pub fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn exec(&self, input: UpdateTodoInput) -> Result<(), UpdateTodoError> {
        let todo_entity = self
            .todo_repository
            .find(input.id)
            .await
            .map_err(|err| match err {
                FindError::NotFound => UpdateTodoError::NotFound,
                FindError::Internal(err) => UpdateTodoError::Repository(err),
            })?;

        let updated_todo_entity = TodoEntity {
            title: input.title.clone(),
            description: input.description,
            todo_at: input.todo_at,
            status: input.status,
            updated_at: DateTime::new(),
            ..todo_entity
        };

        self.todo_repository
            .update(updated_todo_entity)
            .await
            .map_err(|err| match err {
                UpdateError::NotFound => UpdateTodoError::NotFound,
                UpdateError::DuplicatedTitle => UpdateTodoError::DuplicatedTitle(input.title),
                UpdateError::Internal(err) => UpdateTodoError::Repository(err),
            })
    }
}
