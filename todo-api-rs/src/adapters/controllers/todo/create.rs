use crate::adapters::dtos::todo::create::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput};
use crate::application::functions::todo::create::{create_todo, CreateTodoContext};
use crate::application::repositories::todo::create::Create;

pub struct CreateController<Repo: Create> {
    repository: Repo,
}

impl<Repo: Create> CreateController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<TodoPresenter, RunError>
    where
        Req: Parse<CreateTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = CreateTodoContext::new(&self.repository);
        let todo = create_todo(ctx, input).await.map_err(|err| match err {
            CreateTodoError::Repository(err) => RunError::Repository(err),
        })?;

        Ok(TodoPresenter::from(todo))
    }
}
