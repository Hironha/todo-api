use sqlx::types::time::{Date, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
