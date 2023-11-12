use sqlx::{Error as SqlxError, PgConnection};

use crate::application::repositories::tag::delete::DeleteError;
use crate::domain::types::Id;

pub(super) async fn delete_tag(conn: &mut PgConnection, id: Id) -> Result<(), DeleteError> {
    let q = r#"DELETE FROM tag WHERE id = $1"#;

    sqlx::query(q)
        .bind(id.into_uuid())
        .fetch_one(conn)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => DeleteError::NotFound,
            _ => DeleteError::from_err(err),
        })?;

    Ok(())
}
