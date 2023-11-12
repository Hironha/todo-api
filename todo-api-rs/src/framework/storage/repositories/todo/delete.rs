use sqlx::types::Uuid;
use sqlx::{Error as SqlxError, PgConnection};

use crate::application::repositories::todo::delete::DeleteError;
use crate::domain::types::Id;

pub(super) async fn delete_todo(conn: &mut PgConnection, id: Id) -> Result<(), DeleteError> {
    let delete_q = "DELETE FROM todo WHERE id = $1 RETURNING id";

    sqlx::query_scalar::<_, Uuid>(delete_q)
        .bind(id.into_uuid())
        .fetch_one(conn)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => DeleteError::NotFound,
            _ => DeleteError::from_err(err),
        })?;

    Ok(())
}
