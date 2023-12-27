use crate::adapters::dtos::tag::delete::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::delete::DeleteTagUseCase;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteController<T>
where
    T: TagRepository + Clone,
{
    tag_repository: T,
}

impl<T> DeleteController<T>
where
    T: TagRepository + Clone,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<(), RunError>
    where
        R: Parse<Id, ParseError>,
    {
        let tag_id = req.parse().map_err(RunError::Parsing)?;

        DeleteTagUseCase::new(self.tag_repository.clone())
            .exec(tag_id)
            .await
            .map_err(RunError::Deleting)
    }
}
