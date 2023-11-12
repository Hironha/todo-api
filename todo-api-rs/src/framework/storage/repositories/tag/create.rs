use sqlx::PgConnection;

use crate::application::repositories::tag::create::{CreateError, CreatePayload};
use crate::framework::storage::models::tag::TagModel;

pub(super) async fn create_tag(
    conn: &mut PgConnection,
    payload: CreatePayload,
) -> Result<TagModel, CreateError> {
    let q = r#"
        INSERT INTO tag (id, name, description, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, created_at, updated_at
    "#;

    sqlx::query_as::<_, TagModel>(q)
        .bind(payload.id.into_uuid())
        .bind(payload.name.into_string())
        .bind(payload.description.into_opt_string())
        .bind(payload.created_at.into_date_time())
        .bind(payload.updated_at.into_date_time())
        .fetch_one(conn)
        .await
        .map_err(CreateError::from_err)
}
