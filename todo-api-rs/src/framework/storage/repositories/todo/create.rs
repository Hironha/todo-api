use sqlx::PgConnection;

use crate::application::repositories::todo::create::{CreateError, CreatePayload};
use crate::domain::types::Id;
use crate::framework::storage::models::todo::TodoModel;

pub(super) async fn create_todo(
    conn: &mut PgConnection,
    payload: CreatePayload,
) -> Result<TodoModel, CreateError> {
    let insert_q = r#"
        INSERT INTO todo (id, title, description, todo_at, done, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, title, description, todo_at, done, created_at, updated_at
    "#;

    sqlx::query_as::<_, TodoModel>(insert_q)
        .bind(Id::new().into_uuid())
        .bind(payload.title.into_string())
        .bind(payload.description.into_opt_string())
        .bind(payload.todo_at.map(|at| at.into_date()))
        .bind(payload.done)
        .bind(payload.created_at.into_date_time())
        .bind(payload.updated_at.into_date_time())
        .fetch_one(conn.as_mut())
        .await
        .map_err(CreateError::from_err)
}
