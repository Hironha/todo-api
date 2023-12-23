use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput};
use crate::application::repositories::tag::{FindError, TagRepository, UpdateError};
use crate::domain::entities::tag::TagEntity;

pub async fn update_tag<T>(
    ctx: UpdateTagContext<'_, T>,
    input: UpdateTagInput,
) -> Result<TagEntity, UpdateTagError>
where
    T: TagRepository,
{
    let tag_entity = ctx
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

    ctx.tag_repository
        .update(updated_tag_entity)
        .await
        .map_err(|err| match err {
            UpdateError::NotFound => UpdateTagError::NotFound,
            UpdateError::DuplicatedName => UpdateTagError::DuplicatedName(input.name),
            UpdateError::Internal(err) => UpdateTagError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct UpdateTagContext<'a, T>
where
    T: TagRepository,
{
    pub tag_repository: &'a T,
}
