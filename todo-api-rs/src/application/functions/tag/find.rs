use crate::application::dtos::tag::find::{FindTagError, FindTagInput};
use crate::application::repositories::tag::find::{Find, FindError};
use crate::domain::entities::tag::TagEntity;

pub async fn find_tag<Repo: Find>(
    ctx: FindTagContext<'_, Repo>,
    FindTagInput(id): FindTagInput,
) -> Result<TagEntity, FindTagError> {
    ctx.repository.find(id).await.map_err(|err| match err {
        FindError::NotFound => FindTagError::NotFound,
        FindError::Internal => FindTagError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct FindTagContext<'a, Repo: Find> {
    repository: &'a Repo,
}

impl<'a, Repo: Find> FindTagContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
