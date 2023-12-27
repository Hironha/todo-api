use crate::application::dtos::tag::create::{CreateTagError, CreateTagInput};
use crate::application::repositories::tag::{CreateError, TagRepository};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::{DateTime, Id};

#[derive(Debug)]
pub struct CreateTagUseCase<T: TagRepository> {
    tag_repository: T,
}

impl<T: TagRepository> CreateTagUseCase<T> {
    pub fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn exec(&self, input: CreateTagInput) -> Result<TagEntity, CreateTagError> {
        let current_dt = DateTime::new();
        let tag_entity = TagEntity {
            id: Id::new(),
            name: input.name.clone(),
            description: input.description,
            created_at: current_dt,
            updated_at: current_dt,
        };

        self.tag_repository
            .create(tag_entity)
            .await
            .map_err(|err| match err {
                CreateError::DuplicatedName => CreateTagError::DuplicatedName(input.name),
                CreateError::Internal(err) => CreateTagError::Repository(err),
            })
    }
}
