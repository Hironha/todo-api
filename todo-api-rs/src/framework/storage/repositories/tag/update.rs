use sqlx::{Error as SqlxError, PgConnection};

use crate::application::repositories::tag::update::{UpdateError, UpdatePayload};
use crate::framework::storage::models::tag::TagModel;

pub(super) async fn update_tag(
    conn: &mut PgConnection,
    payload: UpdatePayload,
) -> Result<TagModel, UpdateError> {
    let q = r#"
        UPDATE tag
        SET name = $1, description = $2, updated_at = $3
        WHERE id = $4
        RETURNING id, name, description, created_at, updated_at
    "#;

    sqlx::query_as::<_, TagModel>(q)
        .bind(payload.name.into_string())
        .bind(payload.description.into_opt_string())
        .bind(payload.updated_at.into_date_time())
        .bind(payload.id.into_uuid())
        .fetch_one(conn)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => UpdateError::NotFound,
            _ => UpdateError::from_err(err)
        })
}
