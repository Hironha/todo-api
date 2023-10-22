use sqlx::{Error as SqlxError, Executor, Postgres};

use crate::application::repositories::tag::find::FindError;
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

pub(super) async fn find_tag(
    executor: impl Executor<'_, Database = Postgres>,
    id: Id,
) -> Result<TagModel, FindError> {
    let q = r#"
        SELECT id, name, description, created_at, updated_at
        FROM tag
        WHERE id = $1
    "#;

    sqlx::query_as::<_, TagModel>(q)
        .bind(id.into_uuid())
        .fetch_one(executor)
        .await
        .map_err(|err| match err {
            SqlxError::RowNotFound => FindError::NotFound,
            _ => {
                tracing::error!("find tag error: {err:?}");
                FindError::Internal
            }
        })
}
