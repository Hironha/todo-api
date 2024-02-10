use crate::application::dtos::tag::list_all::{AllTagsList, ListAllTagsError, ListAllTagsOutput};
use crate::application::repositories::tag::{ListAllError, TagRepository};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct ListAllTagsUseCase<T> {
    repository: T,
}

impl<T: TagRepository> ListAllTagsUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TagRepository> UseCase<(), ListAllTagsOutput> for ListAllTagsUseCase<T> {
    async fn exec(self, _: ()) -> ListAllTagsOutput {
        let entities = self.repository.list_all().await.map_err(|err| match err {
            ListAllError::Internal(err) => ListAllTagsError::Internal(err),
        })?;

        Ok(AllTagsList {
            count: entities.len(),
            items: entities,
        })
    }
}
