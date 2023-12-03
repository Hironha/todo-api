use crate::application::dtos::tag::list::ListTagError;
use crate::application::repositories::tag::{ListAllError, TagRepository};
use crate::domain::entities::tag::TagEntity;

pub async fn list_tag<T>(ctx: ListTagContext<'_, T>) -> Result<Vec<TagEntity>, ListTagError>
where
    T: TagRepository,
{
    ctx.tag_repository.list_all().await.map_err(|err| match err {
        ListAllError::Internal(err) => ListTagError::Repository(err),
    })
}

#[derive(Clone, Debug)]
pub struct ListTagContext<'a, T>
where
    T: TagRepository,
{
    pub tag_repository: &'a T,
}
