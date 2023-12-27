use crate::adapters::dtos::todo::delete::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::delete::DeleteTodoUseCase;
use crate::domain::types::Id;

pub struct DeleteController<T>
where
    T: TodoRepository + Clone,
{
    todo_repository: T,
}

impl<T: TodoRepository> DeleteController<T>
where
    T: TodoRepository + Clone,
{
    pub const fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<(), RunError>
    where
        R: Parse<Id, ParseError>,
    {
        let todo_id = req.parse().map_err(RunError::Parsing)?;

        DeleteTodoUseCase::new(self.todo_repository.clone())
            .exec(todo_id)
            .await
            .map_err(RunError::Deleting)
    }
}
