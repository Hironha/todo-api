use crate::adapters::dtos::todo::find::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::find::FindTodoInput;
use crate::application::functions::todo::find::{find_todo, FindTodoContext};
use crate::application::repositories::todo::find::Find;

#[derive(Clone, Debug)]
pub struct FindController<Repo: Find> {
    repository: Repo,
}

impl<Repo: Find> FindController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<TodoPresenter, RunError>
    where
        Req: Parse<FindTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = FindTodoContext::new(&self.repository);
        let todo = find_todo(ctx, input).await.map_err(RunError::Finding)?;

        Ok(TodoPresenter::from(todo))
    }
}
