use sqlx::{Error as SqlxError, PgConnection};

use crate::application::repositories::tag::update::{UpdateError, UpdatePayload};
use crate::framework::storage::models::tag::TagModel;

pub(super) async fn update_tag(
    conn: &mut PgConnection,
    payload: UpdatePayload,
) -> Result<TagModel, UpdateError> {
    let q = r#"
        UPDATE tag
        SET name = ($1), description = ($2)
        WHERE id = ($3)
        RETURNING id, name, description, created_at, updated_at
    "#;

    sqlx::query_as::<_, TagModel>(q)
        .bind(payload.id.into_uuid())
        .bind(payload.name.into_string())
        .bind(payload.description.into_opt_string())
        .fetch_one(conn)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => UpdateError::NotFound,
            _ => {
                tracing::error!("update tag error {err:?}");
                UpdateError::Internal
            }
        })
}
