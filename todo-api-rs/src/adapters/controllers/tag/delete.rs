use std::error::Error;

use crate::adapters::dtos::tag::delete::ParseError;
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

    pub async fn run<R>(&self, req: R) -> Result<(), Box<dyn Error>>
    where
        R: Parse<Id, ParseError>,
    {
        let tag_id = req.parse()?;

        DeleteTagUseCase::new(self.tag_repository.clone())
            .exec(tag_id)
            .await
            .map_err(Box::from)
    }
}
