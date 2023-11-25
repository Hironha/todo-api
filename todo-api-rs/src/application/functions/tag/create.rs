use crate::application::dtos::tag::create::{CreateTagError, CreateTagInput};
use crate::application::repositories::tag::create::{Create, CreateError, CreatePayload};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

pub async fn create_tag<Repo: Create>(
    ctx: CreateTagContext<'_, Repo>,
    input: CreateTagInput,
) -> Result<TagEntity, CreateTagError> {
    let payload = CreatePayload {
        id: Id::new(),
        name: input.name,
        description: input.description,
    };

    ctx.repository
        .create(payload)
        .await
        .map_err(|err| match err {
            CreateError::Internal(err) => CreateTagError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct CreateTagContext<'a, Repo: Create> {
    repository: &'a Repo,
}

impl<'a, Repo: Create> CreateTagContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
