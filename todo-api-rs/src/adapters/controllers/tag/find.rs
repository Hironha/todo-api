use crate::adapters::dtos::tag::find::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::find::FindTagUseCase;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindController<T>
where
    T: TagRepository + Clone,
{
    tag_repository: T,
}

impl<T> FindController<T>
where
    T: TagRepository + Clone,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TagPresenter, RunError>
    where
        R: Parse<Id, ParseError>,
    {
        let tag_id = req.parse().map_err(RunError::Parsing)?;

        FindTagUseCase::new(self.tag_repository.clone())
            .exec(tag_id)
            .await
            .map(TagPresenter::from)
            .map_err(RunError::Finding)
    }
}
