use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct TodoTag {
    pub todo_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
