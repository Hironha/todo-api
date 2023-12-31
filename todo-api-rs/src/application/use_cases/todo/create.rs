use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput};
use crate::application::repositories::todo::{CreateError, TodoRepository};
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::{DateTime, Id};

#[derive(Debug)]
pub struct CreateTodoUseCase<T: TodoRepository> {
    todo_repository: T,
}

impl<T: TodoRepository> CreateTodoUseCase<T> {
    pub fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn exec(&self, input: CreateTodoInput) -> Result<TodoEntity, CreateTodoError> {
        let current_dt = DateTime::new();
        let todo_entity = TodoEntity {
            id: Id::new(),
            title: input.title.clone(),
            description: input.description,
            status: input.status,
            todo_at: input.todo_at,
            tags: Vec::new(),
            created_at: current_dt,
            updated_at: current_dt,
        };

        self.todo_repository
            .create(todo_entity)
            .await
            .map_err(|err| match err {
                CreateError::DuplicatedTitle => CreateTodoError::DuplicatedTitle(input.title),
                CreateError::Internal(err) => CreateTodoError::Repository(err),
            })
    }
}
