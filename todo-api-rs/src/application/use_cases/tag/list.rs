use crate::application::dtos::tag::list::ListTagError;
use crate::application::repositories::tag::{ListAllError, TagRepository};
use crate::domain::entities::tag::TagEntity;

#[derive(Debug)]
pub struct ListTagsUseCase<T: TagRepository> {
    tag_repository: T,
}

impl<T: TagRepository> ListTagsUseCase<T> {
    pub fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn exec(&self) -> Result<Vec<TagEntity>, ListTagError> {
        self.tag_repository
            .list_all()
            .await
            .map_err(|err| match err {
                ListAllError::Internal(err) => ListTagError::Repository(err),
            })
    }
}
