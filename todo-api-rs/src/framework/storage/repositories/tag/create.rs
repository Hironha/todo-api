use sqlx::PgConnection;

use crate::application::repositories::tag::create::CreateError;
use crate::domain::entities::tag::TagEntity;
use crate::framework::storage::models::tag::TagModel;

pub(super) async fn create_tag(
    conn: &mut PgConnection,
    entity: TagEntity,
) -> Result<TagModel, CreateError> {
    let q = r#"
        INSERT INTO tag (id, name, description, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, created_at, updated_at
    "#;

    sqlx::query_as::<_, TagModel>(q)
        .bind(entity.id.into_uuid())
        .bind(entity.name.into_string())
        .bind(entity.description.into_opt_string())
        .bind(entity.created_at.into_date_time())
        .bind(entity.updated_at.into_date_time())
        .fetch_one(conn)
        .await
        .map_err(|err| {
            tracing::error!("create tag error: {err:?}");
            CreateError::Internal
        })
}
