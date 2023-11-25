use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput};
use crate::application::repositories::tag::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::tag::TagEntity;

pub async fn update_tag<Repo: Update>(
    ctx: UpdateTagContext<'_, Repo>,
    input: UpdateTagInput,
) -> Result<TagEntity, UpdateTagError> {
    let payload = UpdatePayload {
        id: input.id,
        name: input.name,
        description: input.description,
    };

    ctx.repository
        .update(payload)
        .await
        .map_err(|err| match err {
            UpdateError::NotFound => UpdateTagError::NotFound,
            UpdateError::Internal(err) => UpdateTagError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct UpdateTagContext<'a, Repo: Update> {
    repository: &'a Repo,
}

impl<'a, Repo: Update> UpdateTagContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
