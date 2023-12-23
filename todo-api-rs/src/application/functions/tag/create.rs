use crate::application::dtos::tag::create::{CreateTagError, CreateTagInput};
use crate::application::repositories::tag::{CreateError, TagRepository};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::{DateTime, Id};

pub async fn create_tag<T>(
    ctx: CreateTagContext<'_, T>,
    input: CreateTagInput,
) -> Result<TagEntity, CreateTagError>
where
    T: TagRepository,
{
    let current_dt = DateTime::new();
    let tag_entity = TagEntity {
        id: Id::new(),
        name: input.name.clone(),
        description: input.description,
        created_at: current_dt,
        updated_at: current_dt,
    };

    ctx.tag_repository
        .create(tag_entity)
        .await
        .map_err(|err| match err {
            CreateError::DuplicatedName => CreateTagError::DuplicatedName(input.name),
            CreateError::Internal(err) => CreateTagError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct CreateTagContext<'a, T: TagRepository> {
    pub tag_repository: &'a T,
}
