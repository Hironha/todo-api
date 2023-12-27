use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput};
use crate::application::repositories::tag::{FindError, TagRepository, UpdateError};
use crate::domain::entities::tag::TagEntity;

#[derive(Debug)]
pub struct UpdateTagUseCase<T: TagRepository> {
    tag_repository: T,
}

impl<T: TagRepository> UpdateTagUseCase<T> {
    pub fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn exec(&self, input: UpdateTagInput) -> Result<TagEntity, UpdateTagError> {
        let tag_entity = self
            .tag_repository
            .find(input.id)
            .await
            .map_err(|err| match err {
                FindError::NotFound => UpdateTagError::NotFound,
                FindError::Internal(err) => UpdateTagError::Repository(err),
            })?;

        let updated_tag_entity = TagEntity {
            name: input.name.clone(),
            description: input.description,
            ..tag_entity
        };

        self.tag_repository
            .update(updated_tag_entity)
            .await
            .map_err(|err| match err {
                UpdateError::NotFound => UpdateTagError::NotFound,
                UpdateError::DuplicatedName => UpdateTagError::DuplicatedName(input.name),
                UpdateError::Internal(err) => UpdateTagError::Repository(err),
            })
    }
}
