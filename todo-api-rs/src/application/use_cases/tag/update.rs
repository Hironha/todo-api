use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput, UpdateTagOutput};
use crate::application::repositories::tag::{FindError, TagRepository, UpdateError};
use crate::domain::entities::tag::TagEntity;
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct UpdateTagUseCase<T> {
    repository: T,
}

impl<T: TagRepository> UpdateTagUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TagRepository> UseCase<UpdateTagInput, UpdateTagOutput> for UpdateTagUseCase<T> {
    async fn exec(mut self, input: UpdateTagInput) -> UpdateTagOutput {
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
