use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;

use crate::application::repositories::tag::create::{Create, CreateError, CreateTagPayload};
use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

#[derive(Clone)]
pub struct TagStore {
    pool: Pool<Postgres>,
}

impl TagStore {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Create for TagStore {
    async fn create(&self, payload: CreateTagPayload) -> Result<TagEntity, CreateError> {
        let q = r#"
            INSERT INTO tag (id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, created_at, updated_at
        "#;

        let current_date_time = OffsetDateTime::now_utc();
        let model = sqlx::query_as::<_, TagModel>(q)
            .bind(Id::new().into_uuid())
            .bind(payload.name.into_string())
            .bind(payload.description.into_opt_string())
            .bind(current_date_time)
            .bind(current_date_time)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("create tag repository error {err:?}");
                CreateError::Internal
            })?;

        tag_model_to_entity(model).map_err(|_| CreateError::Internal)
    }
}

fn tag_model_to_entity(model: TagModel) -> Result<TagEntity, ()> {
    let name = Name::new(model.name).map_err(|err| {
        let msg = err.description();
        tracing::error!("tag model name incompatible with tag entity name: {msg}");
    })?;
    let description = Description::new(model.description).map_err(|err| {
        let msg = err.description();
        tracing::error!("tag model description incompatible with tag entity description: {msg}");
    })?;

    Ok(TagEntity {
        id: model.id.into(),
        name,
        description,
        created_at: model.created_at.into(),
        updated_at: model.updated_at.into(),
    })
}
