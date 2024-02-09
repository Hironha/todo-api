use crate::application::dtos::tag::find::FindTagError;
use crate::application::repositories::tag::{FindError, TagRepository};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

#[derive(Debug)]
pub struct FindTagUseCase<T> {
    repository: T,
}

impl<T: TagRepository> FindTagUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn exec(&self, tag_id: Id) -> Result<TagEntity, FindTagError> {
        self.repository.find(tag_id).await.map_err(|err| match err {
            FindError::NotFound => FindTagError::NotFound,
            FindError::Internal(err) => FindTagError::Internal(err),
        })
    }
}
