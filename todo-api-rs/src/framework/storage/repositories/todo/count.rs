use sqlx::{Error as SqlxError, Executor, Postgres, QueryBuilder};

pub(super) async fn count_todo(
    executor: impl Executor<'_, Database = Postgres>,
    filters: CountTodoFilters<'_>,
) -> Result<i64, SqlxError> {
    let mut count_q = QueryBuilder::<'_, Postgres>::new("SELECT COUNT(*) FROM todo");

    if let Some(title) = filters.title {
        count_q
            .push(" WHERE title ILIKE %")
            .push_bind(title)
            .push("%");
    }

    count_q
        .build_query_scalar::<i64>()
        .fetch_one(executor)
        .await
}

#[derive(Clone, Debug)]
pub(super) struct CountTodoFilters<'t> {
    pub(super) title: Option<&'t str>,
}
