use crate::adapters::dtos::todo::delete::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::delete::DeleteTodoInput;
use crate::application::functions::todo::delete::{delete_todo, DeleteTodoContext};
use crate::application::repositories::todo::delete::Delete;

pub struct DeleteController<Repo: Delete> {
    repository: Repo,
}

impl<Repo: Delete> DeleteController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<(), RunError>
    where
        Req: Parse<DeleteTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = DeleteTodoContext::new(&self.repository);

        delete_todo(ctx, input).await.map_err(RunError::Deleting)
    }
}
