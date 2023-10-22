use sqlx::types::time::OffsetDateTime;
use sqlx::{Executor, Postgres};

use crate::application::repositories::tag::create::{CreateError, CreatePayload};
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

pub(super) async fn create_tag(
    executor: impl Executor<'_, Database = Postgres>,
    payload: CreatePayload,
) -> Result<TagModel, CreateError> {
    let q = r#"
        INSERT INTO tag (id, name, description, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, created_at, updated_at
    "#;

    let current_datetime = OffsetDateTime::now_utc();
    sqlx::query_as::<_, TagModel>(q)
        .bind(Id::new().into_uuid())
        .bind(payload.name.into_string())
        .bind(payload.description.into_opt_string())
        .bind(current_datetime)
        .bind(current_datetime)
        .fetch_one(executor)
        .await
        .map_err(|err| {
            tracing::error!("create tag error: {err:?}");
            CreateError::Internal
        })
}
