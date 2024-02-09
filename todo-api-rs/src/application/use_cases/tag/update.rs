use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput};
use crate::application::repositories::tag::{FindError, TagRepository, UpdateError};
use crate::domain::entities::tag::TagEntity;

#[derive(Debug)]
pub struct UpdateTagUseCase<T> {
    repository: T,
}

impl<T: TagRepository> UpdateTagUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn exec(&self, input: UpdateTagInput) -> Result<TagEntity, UpdateTagError> {
        let tag_entity = self
            .repository
            .find(input.id)
            .await
            .map_err(|err| match err {
                FindError::NotFound => UpdateTagError::NotFound,
                FindError::Internal(err) => UpdateTagError::Internal(err),
            })?;

        let updated_tag_entity = TagEntity {
            name: input.name.clone(),
            description: input.description,
            ..tag_entity
        };

        self.repository
            .update(updated_tag_entity)
            .await
            .map_err(|err| match err {
                UpdateError::NotFound => UpdateTagError::NotFound,
                UpdateError::DuplicatedName => UpdateTagError::DuplicatedName(input.name),
                UpdateError::Internal(err) => UpdateTagError::Internal(err),
            })
    }
}
