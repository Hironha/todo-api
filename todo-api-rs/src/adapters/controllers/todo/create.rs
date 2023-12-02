use crate::adapters::dtos::todo::create::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::create::CreateTodoInput;
use crate::application::functions::todo::create::{create_todo, CreateTodoContext};
use crate::application::repositories::todo::TodoRepository;

pub struct CreateController<T: TodoRepository>
where
    T: TodoRepository,
{
    repository: T,
}

impl<T: TodoRepository> CreateController<T> {
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TodoPresenter, RunError>
    where
        R: Parse<CreateTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = CreateTodoContext {
            todo_repository: &self.repository,
        };

        let todo = create_todo(ctx, input).await.map_err(RunError::Creating)?;

        Ok(TodoPresenter::from(todo))
    }
}
