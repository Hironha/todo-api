use sqlx::types::Uuid;
use sqlx::{Error as SqlxError, Executor, Postgres};

use crate::application::repositories::todo::delete::DeleteError;
use crate::domain::types::Id;

pub(super) async fn delete_todo(
    executor: impl Executor<'_, Database = Postgres>,
    id: Id,
) -> Result<(), DeleteError> {
    let delete_q = r#"
        DELETE FROM todo
        WHERE id = $1
        RETURNING id
    "#;

    sqlx::query_scalar::<_, Uuid>(delete_q)
        .bind(id.into_uuid())
        .fetch_one(executor)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => DeleteError::NotFound,
            _ => {
                tracing::error!("delete todo failed deleting {err:?}");
                DeleteError::Internal
            }
        })?;

    Ok(())
}
