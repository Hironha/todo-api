use sqlx::{Executor, Postgres};
use time::OffsetDateTime;

use crate::application::repositories::todo::create::{CreateError, CreatePayload};
use crate::domain::types::Id;
use crate::framework::storage::models::todo::TodoModel;

pub(super) async fn create_todo(
    executor: impl Executor<'_, Database = Postgres>,
    payload: CreatePayload,
) -> Result<TodoModel, CreateError> {
    let insert_q = r#"
        INSERT INTO todo (id, title, description, todo_at, done, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, title, description, todo_at, done, created_at, updated_at
    "#;

    let current_date_time = OffsetDateTime::now_utc();
    sqlx::query_as::<_, TodoModel>(insert_q)
        .bind(Id::new().into_uuid())
        .bind(payload.title.into_string())
        .bind(payload.description.into_opt_string())
        .bind(payload.todo_at.map(|at| at.into_date()))
        .bind(payload.done)
        .bind(current_date_time)
        .bind(current_date_time)
        .fetch_one(executor)
        .await
        .map_err(|err| {
            tracing::error!("create todo failed creating {err:?}");
            CreateError::Internal
        })
}
