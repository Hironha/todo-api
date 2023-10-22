use sqlx::{Executor, Postgres};

use crate::application::repositories::tag::list::ListError;
use crate::framework::storage::models::tag::TagModel;

pub(super) async fn list_tag(
    executor: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<TagModel>, ListError> {
    let q = r#"
        SELECT id, name, description, created_at, updated_at
        FROM tag
    "#;

    sqlx::query_as::<_, TagModel>(q)
        .fetch_all(executor)
        .await
        .map_err(|err| {
            tracing::error!("list tag error: {err:?}");
            ListError::Internal
        })
}
