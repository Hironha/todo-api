use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput, CreateTodoOutput};
use crate::application::repositories::todo::{CreateError, TodoRepository};
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::{DateTime, Id};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct CreateTodoUseCase<T> {
    repository: T,
}

impl<T: TodoRepository> CreateTodoUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TodoRepository> UseCase<CreateTodoInput, CreateTodoOutput> for CreateTodoUseCase<T> {
    async fn exec(mut self, input: CreateTodoInput) -> CreateTodoOutput {
        let current_dt = DateTime::now();
        let todo_entity = TodoEntity {
            id: Id::new(),
            title: input.title.clone(),
            description: input.description,
            status: input.status,
            todo_at: input.todo_at,
            created_at: current_dt,
            updated_at: current_dt,
        };

        self.repository
            .create(todo_entity)
            .await
            .map_err(|err| match err {
                CreateError::DuplicatedTitle => CreateTodoError::DuplicatedTitle(input.title),
                CreateError::Internal(err) => CreateTodoError::Internal(err),
            })
    }
}
