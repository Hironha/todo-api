use crate::adapters::dtos::todo::delete::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::delete::DeleteTodoInput;
use crate::application::functions::todo::delete::{delete_todo, DeleteTodoContext};
use crate::application::repositories::todo::TodoRepository;

pub struct DeleteController<T>
where
    T: TodoRepository,
{
    repository: T,
}

impl<T: TodoRepository> DeleteController<T> {
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<(), RunError>
    where
        R: Parse<DeleteTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = DeleteTodoContext {
            todo_repository: &self.repository,
        };

        delete_todo(ctx, input).await.map_err(RunError::Deleting)
    }
}
