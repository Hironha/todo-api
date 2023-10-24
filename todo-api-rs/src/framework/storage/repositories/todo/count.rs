use sqlx::{Error as SqlxError, PgConnection, Postgres, QueryBuilder};

pub(super) async fn count_todo(
    conn: &mut PgConnection,
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
        .fetch_one(conn.as_mut())
        .await
}

#[derive(Clone, Debug)]
pub(super) struct CountTodoFilters<'t> {
    pub(super) title: Option<&'t str>,
}
