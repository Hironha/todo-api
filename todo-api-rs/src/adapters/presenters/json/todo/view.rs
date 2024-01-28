use serde::Serialize;

use crate::adapters::presenters::json::tag::TagView;
use crate::domain::entities::todo::TodoEntity;

/// Presentable format of `TodoEntity`
#[derive(Clone, Debug, Serialize)]
pub struct TodoView {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    /// `Date` in UTC YYYY-MM-DD format
    #[serde(rename(serialize = "todoAt"))]
    pub todo_at: Option<String>,
    pub tags: Vec<TagView>,
    /// `Date` in `RFC-3339` format
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
    /// `Date` in `RFC-3339` format
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
}

impl From<TodoEntity> for TodoView {
    fn from(entity: TodoEntity) -> Self {
        let tag_views: Vec<TagView> = entity.tags.into_iter().map(TagView::from).collect();

        Self {
            id: entity.id.to_string(),
            title: entity.title.into_inner(),
            description: entity.description.map(|d| d.into_inner()),
            status: entity.status.to_string(),
            todo_at: entity.todo_at.map(|at| at.to_ymd()),
            tags: tag_views,
            created_at: entity.created_at.to_rfc3339(),
            updated_at: entity.updated_at.to_rfc3339(),
        }
    }
}
