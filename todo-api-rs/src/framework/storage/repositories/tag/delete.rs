use sqlx::{Error as SqlxError, Executor, Postgres};

use crate::application::repositories::tag::delete::DeleteError;
use crate::domain::types::Id;

pub(super) async fn delete_tag(
    executor: impl Executor<'_, Database = Postgres>,
    id: Id,
) -> Result<(), DeleteError> {
    let q = r#"DELETE FROM tag WHERE id = $1"#;

    sqlx::query(q)
        .bind(id.into_uuid())
        .fetch_one(executor)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => DeleteError::NotFound,
            _ => {
                tracing::error!("delete tag err {err:?}");
                DeleteError::Internal
            }
        })?;

    Ok(())
}
