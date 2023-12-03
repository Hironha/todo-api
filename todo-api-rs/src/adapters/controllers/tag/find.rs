use crate::adapters::dtos::tag::find::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::find::FindTagInput;
use crate::application::functions::tag::find::{find_tag, FindTagContext};
use crate::application::repositories::tag::TagRepository;

#[derive(Clone, Debug)]
pub struct FindController<T> {
    tag_repository: T,
}

impl<T> FindController<T>
where
    T: TagRepository,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TagPresenter, RunError>
    where
        R: Parse<FindTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = FindTagContext {
            tag_repository: &self.tag_repository,
        };

        find_tag(ctx, input)
            .await
            .map(TagPresenter::from)
            .map_err(RunError::Finding)
    }
}
