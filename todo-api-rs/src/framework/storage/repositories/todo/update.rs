use sqlx::types::time::OffsetDateTime;
use sqlx::{Error as SqlxError, Executor, Postgres};

use crate::application::repositories::todo::update::{UpdateError, UpdatePayload};
use crate::framework::storage::models::todo::TodoModel;

pub(super) async fn update_todo(
    executor: impl Executor<'_, Database = Postgres>,
    payload: UpdatePayload,
) -> Result<TodoModel, UpdateError> {
    let q = r#"
        UPDATE todo
        SET title = $1, description = $2, todo_at = $3, done = $4, updated_at = $5
        WHERE id = $6
        RETURNING id, title, description, todo_at, done, created_at, updated_at
    "#;

    sqlx::query_as::<_, TodoModel>(q)
        .bind(payload.title.into_string())
        .bind(payload.description.into_opt_string())
        .bind(payload.todo_at.map(|at| at.into_date()))
        .bind(payload.done)
        .bind(OffsetDateTime::now_utc())
        .bind(payload.id.into_uuid())
        .fetch_one(executor)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => UpdateError::NotFound,
            _ => {
                tracing::error!("update todo repository error: {err:?}");
                UpdateError::Internal
            }
        })
}
