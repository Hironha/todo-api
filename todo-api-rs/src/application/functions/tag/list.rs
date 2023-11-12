use crate::application::dtos::tag::list::ListTagError;
use crate::application::repositories::tag::list::{List, ListError};
use crate::domain::entities::tag::TagEntity;

pub async fn list_tag<Repo: List>(
    ctx: ListTagContext<'_, Repo>,
) -> Result<Vec<TagEntity>, ListTagError> {
    ctx.repository.list().await.map_err(|err| match err {
        ListError::Internal(err) => ListTagError::Repository(err),
    })
}

#[derive(Clone, Debug)]
pub struct ListTagContext<'a, Repo: List> {
    repository: &'a Repo,
}

impl<'a, Repo: List> ListTagContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
