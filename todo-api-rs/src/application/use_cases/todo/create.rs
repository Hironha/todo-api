use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput, CreateTodoOutput};
use crate::application::repositories::todo::{CreateError, TodoRepository};
use crate::domain::entities::todo::TodoEntity;
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
        let entity = TodoEntity::create()
            .title(input.title.clone())
            .status(input.status)
            .description(input.description)
            .todo_at(input.todo_at)
            .build();

        if let Err(err) = self.repository.create(entity.clone()).await {
            return Err(match err {
                CreateError::DuplicatedTitle => CreateTodoError::DuplicatedTitle(input.title),
                CreateError::Internal(src) => CreateTodoError::Internal(src),
            });
        }

        Ok(entity)
    }
}
