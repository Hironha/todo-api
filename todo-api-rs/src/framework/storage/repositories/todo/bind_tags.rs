use sqlx::types::uuid::Uuid;
use sqlx::{Error as SqlxError, PgConnection, Postgres, QueryBuilder, Transaction};

use crate::application::repositories::todo::bind_tags::{BindTagsError, BindTagsPayload};

pub(super) async fn bind_tags(
    trx: &mut Transaction<'_, Postgres>,
    payload: BindTagsPayload,
) -> Result<(), BindTagsError> {
    let todo_id = payload.todo_id.into_uuid();
    let todo_exists = check_todo_exists(trx, todo_id)
        .await
        .map_err(BindTagsError::from_err)?;

    if !todo_exists {
        return Err(BindTagsError::TodoNotFound);
    }

    let delete_relations_q = "DELETE FROM todo_tag WHERE todo_id = $1";
    sqlx::query(delete_relations_q)
        .bind(todo_id)
        .execute(trx.as_mut())
        .await
        .map_err(BindTagsError::from_err)?;

    let tags_id = payload.tags_id.filter(|ids| !ids.is_empty()).map(|ids| {
        ids.into_iter()
            .map(|id| id.into_uuid())
            .collect::<Vec<Uuid>>()
    });

    if let Some(ids) = tags_id.as_deref() {
        let tags_exists = check_tags_exists(trx, ids)
            .await
            .map_err(BindTagsError::from_err)?;

        if !tags_exists {
            return Err(BindTagsError::TagNotFound);
        }

        let current_dt = payload.current_dt.into_date_time();
        let base_bind_tags_q = "INSERT INTO todo_tag (todo_id, tag_id, created_at) ";
        QueryBuilder::<'_, Postgres>::new(base_bind_tags_q)
            .push_values(ids, |mut q, tag_id| {
                q.push_bind(todo_id).push_bind(tag_id).push_bind(current_dt);
            })
            .build()
            .execute(trx.as_mut())
            .await
            .map_err(BindTagsError::from_err)?;
    }

    Ok(())
}

async fn check_todo_exists(conn: &mut PgConnection, todo_id: Uuid) -> Result<bool, SqlxError> {
    let todo_exists_q = "SELECT EXISTS(SELECT 1 FROM todo WHERE id = $1)";
    sqlx::query_scalar::<_, bool>(todo_exists_q)
        .bind(todo_id)
        .fetch_one(conn.as_mut())
        .await
}

async fn check_tags_exists(conn: &mut PgConnection, tags_id: &[Uuid]) -> Result<bool, SqlxError> {
    let count_tags_q = "SELECT COUNT(*) FROM tag WHERE id = ANY($1) ";
    let count = sqlx::query_scalar::<_, i64>(count_tags_q)
        .bind(tags_id)
        .fetch_one(conn)
        .await?;

    let tags_len = i64::try_from(tags_id.len()).unwrap_or(0);
    Ok(count == tags_len)
}
