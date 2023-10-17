use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::application::repositories::todo::list::{ListError, ListPayload};
use crate::framework::storage::models::todo::TodoModel;

pub(super) async fn list_todo(
    pool: &PgPool,
    payload: ListPayload,
) -> Result<ListTodoData, ListError> {
    let limit: i64 = u32::from(payload.per_page).into();
    let page: i64 = u32::from(payload.page).into();
    let offset = (page - 1) * limit;
    let mut count_q = QueryBuilder::<'_, Postgres>::new(r#"SELECT COUNT(*) FROM todo"#);
    let mut list_q = QueryBuilder::<'_, Postgres>::new(
        r#"
            SELECT id, title, description, todo_at, done, created_at, updated_at
            FROM todo
        "#,
    );

    if let Some(title) = payload.title {
        list_q.push(" WHERE title ILIKE ");
        list_q.push_bind(format!("%{title}%"));

        count_q.push(" WHERE title ILIKE ");
        count_q.push_bind(format!("%{title}%"));
    }

    list_q.push(r#" ORDER BY created_at DESC LIMIT "#);
    list_q.push_bind(limit);

    list_q.push(r#" OFFSET "#);
    list_q.push_bind(offset);

    let db_count = count_q
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await
        .map_err(|err| {
            tracing::error!("list todo failed counting {err:?}");
            ListError::Internal
        })?;

    let count = u64::try_from(db_count).map_err(|_| ListError::Internal)?;

    let todo_models = list_q
        .build_query_as::<TodoModel>()
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!("list todo failed {err:?}");
            ListError::Internal
        })?;

    Ok(ListTodoData {
        count,
        items: todo_models,
    })
}

pub(super) struct ListTodoData {
    pub(super) count: u64,
    pub(super) items: Vec<TodoModel>,
}
