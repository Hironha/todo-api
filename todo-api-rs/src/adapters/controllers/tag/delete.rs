use crate::adapters::dtos::tag::delete::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::delete::DeleteTagInput;
use crate::application::functions::tag::delete::{delete_tag, DeleteTagContext};
use crate::application::repositories::tag::TagRepository;

#[derive(Clone, Debug)]
pub struct DeleteController<T> {
    tag_repository: T,
}

impl<T> DeleteController<T>
where
    T: TagRepository,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<(), RunError>
    where
        R: Parse<DeleteTagInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = DeleteTagContext {
            tag_repository: &self.tag_repository,
        };

        delete_tag(ctx, input).await.map_err(RunError::Deleting)
    }
}
