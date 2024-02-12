use serde::Serialize;

use crate::domain::entities::todo::TodoEntity;

/// Presentable format of `TodoEntity`
#[derive(Clone, Debug, Serialize)]
pub struct TodoView {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    /// Date in YYYY-MM-DD UTC format
    #[serde(rename(serialize = "todoAt"))]
    pub todo_at: Option<String>,
    /// Date time with offset in `RFC-3339` format
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: Option<String>,
    /// Date time with offset in `RFC-3339` format
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

impl From<TodoEntity> for TodoView {
    fn from(entity: TodoEntity) -> Self {
        let id = entity.id().to_string();
        let created_at = entity.created_at().map(|dt| dt.to_rfc3339());
        let updated_at = entity.updated_at().map(|dt| dt.to_rfc3339());

        Self {
            id,
            title: entity.title.into_inner(),
            description: entity.description.map(|d| d.into_inner()),
            status: entity.status.to_string(),
            todo_at: entity.todo_at.map(|at| at.to_ymd()),
            created_at,
            updated_at,
        }
    }
}
