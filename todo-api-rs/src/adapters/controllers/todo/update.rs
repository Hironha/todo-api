use crate::adapters::dtos::todo::update::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::update::UpdateTodoInput;
use crate::application::functions::todo::update::{update_todo, UpdateTodoContext};
use crate::application::repositories::todo::TodoRepository;

pub struct UpdateController<T>
where
    T: TodoRepository,
{
    repository: T,
}

impl<T> UpdateController<T>
where
    T: TodoRepository,
{
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn run<R>(self, req: R) -> Result<(), RunError>
    where
        R: Parse<UpdateTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = UpdateTodoContext {
            todo_repository: &self.repository,
        };

        update_todo(ctx, input).await.map_err(RunError::Updating)
    }
}
