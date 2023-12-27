use crate::adapters::dtos::tag::update::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::update::UpdateTagInput;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::update::UpdateTagUseCase;

#[derive(Clone, Debug)]
pub struct UpdateController<T>
where
    T: TagRepository + Clone,
{
    tag_repository: T,
}

impl<T> UpdateController<T>
where
    T: TagRepository + Clone,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TagPresenter, RunError>
    where
        R: Parse<UpdateTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;

        UpdateTagUseCase::new(self.tag_repository.clone())
            .exec(input)
            .await
            .map(TagPresenter::from)
            .map_err(RunError::Updating)
    }
}
