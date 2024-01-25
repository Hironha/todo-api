use crate::application::dtos::tag::list_all::{ListAllTagsError, ListAllTagsOutput};
use crate::application::repositories::tag::{ListAllError, TagRepository};

#[derive(Debug)]
pub struct ListAllTagsUseCase<T: TagRepository> {
    tag_repository: T,
}

impl<T: TagRepository> ListAllTagsUseCase<T> {
    pub fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn exec(&self) -> Result<ListAllTagsOutput, ListAllTagsError> {
        let entities = self
            .tag_repository
            .list_all()
            .await
            .map_err(|err| match err {
                ListAllError::Internal(err) => ListAllTagsError::Internal(err),
            })?;

        Ok(ListAllTagsOutput {
            count: entities.len(),
            items: entities,
        })
    }
}
