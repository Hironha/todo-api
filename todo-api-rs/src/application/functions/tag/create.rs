use crate::application::dtos::tag::create::{CreateTagError, CreateTagInput, CreateTagOutput};
use crate::application::repositories::tag::create::{Create, CreateError, CreateTagPayload};

pub async fn create_tag<S: Create>(
    ctx: CreateTagContext<S>,
    input: CreateTagInput,
) -> CreateTagOutput {
    let payload = CreateTagPayload {
        name: input.name,
        description: input.description,
    };

    match ctx.store.create(payload).await {
        Ok(tag) => CreateTagOutput::ok(tag),
        Err(err) => CreateTagOutput::err(match err {
            CreateError::Internal => CreateTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct CreateTagContext<S: Create> {
    pub store: S,
}
