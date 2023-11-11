use crate::adapters::dtos::todo::update::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::update::UpdateTodoInput;
use crate::application::functions::todo::update::{update_todo, UpdateTodoContext};
use crate::application::repositories::todo::update::Update;

pub struct UpdateController<Repo: Update> {
    repository: Repo,
}

impl<Repo: Update> UpdateController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(self, req: Req) -> Result<TodoPresenter, RunError>
    where
        Req: Parse<UpdateTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = UpdateTodoContext::new(&self.repository);
        let todo = update_todo(ctx, input).await.map_err(RunError::Updating)?;

        Ok(TodoPresenter::from(todo))
    }
}
