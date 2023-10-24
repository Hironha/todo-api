use crate::application::dtos::tag::create::{CreateTagError, CreateTagInput, CreateTagOutput};
use crate::application::repositories::tag::create::{Create, CreateError};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::{DateTime, Id};

pub async fn create_tag<S: Create>(
    ctx: CreateTagContext<'_, S>,
    input: CreateTagInput,
) -> CreateTagOutput {
    let current_dt = DateTime::new();
    let entity = TagEntity {
        id: Id::new(),
        name: input.name,
        description: input.description,
        created_at: current_dt,
        updated_at: current_dt,
    };

    match ctx.repository.create(entity).await {
        Ok(tag) => CreateTagOutput::ok(tag),
        Err(err) => CreateTagOutput::err(match err {
            CreateError::Internal => CreateTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct CreateTagContext<'a, S: Create> {
    repository: &'a S,
}

impl<'a, S: Create> CreateTagContext<'a, S> {
    pub const fn new(repository: &'a S) -> Self {
        Self { repository }
    }
}
