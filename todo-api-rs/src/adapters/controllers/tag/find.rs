use crate::adapters::dtos::tag::find::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::find::{FindTagError, FindTagInput};
use crate::application::functions::tag::find::{find_tag, FindTagContext};
use crate::application::repositories::tag::find::Find;

#[derive(Clone, Debug)]
pub struct FindController<Repo: Find> {
    repository: Repo,
}

impl<Repo: Find> FindController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<TagPresenter, RunError>
    where
        Req: Parse<FindTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = FindTagContext::new(&self.repository);

        find_tag(ctx, input)
            .await
            .map(TagPresenter::from)
            .map_err(|err| match err {
                FindTagError::NotFound => RunError::NotFound,
                FindTagError::Internal => RunError::Internal,
            })
    }
}
