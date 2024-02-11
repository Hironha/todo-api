use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput, UpdateTagOutput};
use crate::application::repositories::tag::{TagRepository, UpdateError, UpdateQuery};
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
        let query = UpdateQuery {
            id: input.id,
            name: input.name.clone(),
            description: input.description,
        };

        self.repository
            .update(query)
            .await
            .map_err(|err| match err {
                UpdateError::NotFound => UpdateTagError::NotFound,
                UpdateError::DuplicatedName => UpdateTagError::DuplicatedName(input.name),
                UpdateError::Internal(err) => UpdateTagError::Internal(err),
            })
    }
}
