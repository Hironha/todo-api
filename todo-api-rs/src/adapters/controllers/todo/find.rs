use crate::adapters::dtos::todo::find::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::find::FindTodoInput;
use crate::application::functions::todo::find::{find_todo, FindTodoContext};
use crate::application::repositories::todo::TodoRepository;

#[derive(Clone, Debug)]
pub struct FindController<T>
where
    T: TodoRepository,
{
    repository: T,
}

impl<T> FindController<T>
where
    T: TodoRepository,
{
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TodoPresenter, RunError>
    where
        R: Parse<FindTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = FindTodoContext {
            todo_repository: &self.repository,
        };

        find_todo(ctx, input)
            .await
            .map(TodoPresenter::from)
            .map_err(RunError::Finding)
    }
}
