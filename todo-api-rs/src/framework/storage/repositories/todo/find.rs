use sqlx::{Error as SqlxError, PgConnection};

use crate::application::repositories::todo::find::FindError;
use crate::domain::types::Id;
use crate::framework::storage::models::todo::TodoModel;

pub(super) async fn find_todo(conn: &mut PgConnection, id: Id) -> Result<TodoModel, FindError> {
    let q = r#"
        SELECT id, title, description, todo_at, done, created_at, updated_at
        FROM todo 
        WHERE id = $1
    "#;

    sqlx::query_as::<_, TodoModel>(q)
        .bind(id.into_uuid())
        .fetch_one(conn)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => FindError::NotFound,
            _ => FindError::from_err(err),
        })
}
