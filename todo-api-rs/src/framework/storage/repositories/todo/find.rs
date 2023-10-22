use sqlx::{Error as SqlxError, Executor, Postgres};

use crate::application::repositories::todo::find::FindError;
use crate::domain::types::Id;
use crate::framework::storage::models::todo::TodoModel;

pub(super) async fn find_todo(
    executor: impl Executor<'_, Database = Postgres>,
    id: Id,
) -> Result<TodoModel, FindError> {
    let q = r#"
        SELECT id, title, description, todo_at, done, created_at, updated_at
        FROM todo 
        WHERE id = $1
    "#;

    sqlx::query_as::<_, TodoModel>(q)
        .bind(id.into_uuid())
        .fetch_one(executor)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => FindError::NotFound,
            _ => {
                tracing::error!("find todo repository error {err:?}");
                FindError::Internal
            }
        })
}
