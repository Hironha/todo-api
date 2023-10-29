use crate::adapters::dtos::tag::update::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput};
use crate::application::functions::tag::update::{update_tag, UpdateTagContext};
use crate::application::repositories::tag::update::Update;

#[derive(Clone, Debug)]
pub struct UpdateController<Repo: Update> {
    repository: Repo,
}

impl<Repo: Update> UpdateController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<TagPresenter, RunError>
    where
        Req: Parse<UpdateTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = UpdateTagContext::new(&self.repository);

        update_tag(ctx, input)
            .await
            .map(TagPresenter::from)
            .map_err(|err| match err {
                UpdateTagError::NotFound => RunError::NotFound,
                UpdateTagError::Internal => RunError::Internal,
            })
    }
}
