use serde::Serialize;

use crate::domain::entities::todo::TodoEntity;

use crate::adapters::views::tag::TagView;

/// Presentable format of `TodoEntity`
#[derive(Clone, Debug, Serialize)]
pub struct TodoPresenter {
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

impl TodoPresenter {
    pub fn from_entity(entity: TodoEntity) -> Self {
        let tag_presenters = entity
            .tags
            .into_iter()
            .map(TagView::from)
            .collect::<Vec<TagView>>();

        Self {
            id: entity.id.to_string(),
            title: entity.title.into_inner(),
            description: entity.description.map(|d| d.into_inner()),
            status: entity.status.to_string(),
            todo_at: entity.todo_at.map(|at| at.to_ymd()),
            tags: tag_presenters,
            created_at: entity.created_at.to_rfc3339(),
            updated_at: entity.updated_at.to_rfc3339(),
        }
    }
}
