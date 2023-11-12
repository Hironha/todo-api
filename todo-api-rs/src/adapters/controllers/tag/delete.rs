use crate::adapters::dtos::tag::delete::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::delete::DeleteTagInput;
use crate::application::functions::tag::delete::{delete_tag, DeleteTagContext};
use crate::application::repositories::tag::delete::Delete;

#[derive(Clone, Debug)]
pub struct DeleteController<Repo: Delete> {
    repository: Repo,
}

impl<Repo: Delete> DeleteController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<(), RunError>
    where
        Req: Parse<DeleteTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = DeleteTagContext::new(&self.repository);

        delete_tag(ctx, input).await.map_err(RunError::Deleting)
    }
}
