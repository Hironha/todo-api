use crate::adapters::dtos::tag::create::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::create::CreateTagInput;
use crate::application::functions::tag::create::{create_tag, CreateTagContext};
use crate::application::repositories::tag::TagRepository;

pub struct CreateController<T> {
    tag_repository: T,
}

impl<T> CreateController<T>
where
    T: TagRepository,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TagPresenter, RunError>
    where
        R: Parse<CreateTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = CreateTagContext {
            tag_repository: &self.tag_repository,
        };

        create_tag(ctx, input)
            .await
            .map(TagPresenter::from)
            .map_err(RunError::Creating)
    }
}
