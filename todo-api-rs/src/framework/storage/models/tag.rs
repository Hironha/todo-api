use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct TagModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
