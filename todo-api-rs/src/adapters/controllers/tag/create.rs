use crate::adapters::dtos::tag::create::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::create::{CreateTagError, CreateTagInput};
use crate::application::functions::tag::create::{create_tag, CreateTagContext};
use crate::application::repositories::tag::create::Create;

pub struct CreateController<Repo: Create> {
    repository: Repo,
}

impl<Repo: Create> CreateController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<TagPresenter, RunError>
    where
        Req: Parse<CreateTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = CreateTagContext::new(&self.repository);

        create_tag(ctx, input)
            .await
            .map(TagPresenter::from)
            .map_err(|err| match err {
                CreateTagError::Internal => RunError::Internal,
            })
    }
}
