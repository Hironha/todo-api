use crate::application::dtos::tag::find::{FindTagError, FindTagInput};
use crate::application::repositories::tag::{FindError, TagRepository};
use crate::domain::entities::tag::TagEntity;

pub async fn find_tag<T>(
    ctx: FindTagContext<'_, T>,
    FindTagInput(id): FindTagInput,
) -> Result<TagEntity, FindTagError>
where
    T: TagRepository,
{
    ctx.tag_repository.find(id).await.map_err(|err| match err {
        FindError::NotFound => FindTagError::NotFound,
        FindError::Internal(err) => FindTagError::Repository(err),
    })
}

#[derive(Clone, Debug)]
pub struct FindTagContext<'a, T>
where
    T: TagRepository,
{
    pub tag_repository: &'a T,
}
