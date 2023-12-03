use crate::adapters::dtos::todo::create::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::create::CreateTodoInput;
use crate::application::functions::todo::create::{create_todo, CreateTodoContext};
use crate::application::repositories::todo::TodoRepository;

pub struct CreateController<T: TodoRepository> {
    todo_repository: T,
}

impl<T: TodoRepository> CreateController<T>
where
    T: TodoRepository,
{
    pub const fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TodoPresenter, RunError>
    where
        R: Parse<CreateTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = CreateTodoContext {
            todo_repository: &self.todo_repository,
        };

        let todo = create_todo(ctx, input).await.map_err(RunError::Creating)?;

        Ok(TodoPresenter::from(todo))
    }
}
