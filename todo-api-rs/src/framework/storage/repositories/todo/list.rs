use sqlx::{PgConnection, Postgres, QueryBuilder};

use crate::application::repositories::todo::list::{ListError, ListPayload};
use crate::framework::storage::models::todo::TodoModel;

pub(super) async fn list_todo(
    conn: &mut PgConnection,
    payload: ListPayload,
) -> Result<Vec<TodoModel>, ListError> {
    let limit: i64 = u32::from(payload.per_page).into();
    let page: i64 = u32::from(payload.page).into();
    let offset = (page - 1) * limit;
    let mut list_q = QueryBuilder::<'_, Postgres>::new(
        r#"
            SELECT id, title, description, todo_at, done, created_at, updated_at
            FROM todo
        "#,
    );

    if let Some(title) = payload.title.map(|t| t.into_string()) {
        list_q
            .push(" WHERE title ILIKE ")
            .push_bind(format!("%{title}%"));
    }

    list_q
        .push(" ORDER BY created_at DESC LIMIT ")
        .push_bind(limit)
        .push(" OFFSET ")
        .push_bind(offset)
        .build_query_as::<TodoModel>()
        .fetch_all(conn)
        .await
        .map_err(|err| {
            tracing::error!("list todo failed {err:?}");
            ListError::Internal
        })
}
