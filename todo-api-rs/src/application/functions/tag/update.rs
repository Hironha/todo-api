use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput, UpdateTagOutput};
use crate::application::repositories::tag::update::{Update, UpdateError, UpdatePayload};
use crate::domain::types::DateTime;

pub async fn update_tag<S: Update>(
    ctx: UpdateTagContext<'_, S>,
    input: UpdateTagInput,
) -> UpdateTagOutput {
    let payload = UpdatePayload {
        id: input.id,
        name: input.name,
        description: input.description,
        updated_at: DateTime::new(),
    };

    match ctx.repository.update(payload).await {
        Ok(tag) => UpdateTagOutput::ok(tag),
        Err(err) => UpdateTagOutput::err(match err {
            UpdateError::NotFound => UpdateTagError::NotFound,
            UpdateError::Internal => UpdateTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct UpdateTagContext<'a, S: Update> {
    repository: &'a S,
}

impl<'a, S: Update> UpdateTagContext<'a, S> {
    pub const fn new(repository: &'a S) -> Self {
        Self { repository }
    }
}
