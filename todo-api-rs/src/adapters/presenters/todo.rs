use serde::Serialize;

use crate::domain::entities::todo::TodoEntity;

use super::tag::TagPresenter;

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
    pub tags: Vec<TagPresenter>,
    /// `Date` in `RFC-3339` format
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
    /// `Date` in `RFC-3339` format
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
}

impl From<TodoEntity> for TodoPresenter {
    fn from(todo: TodoEntity) -> Self {
        let tag_presenters = todo
            .tags
            .into_iter()
            .map(TagPresenter::from)
            .collect::<Vec<TagPresenter>>();

        Self {
            id: todo.id.to_string(),
            title: todo.title.into_string(),
            description: todo.description.map(|d| d.into_string()),
            status: todo.status.into_string(),
            todo_at: todo.todo_at.map(|at| at.to_ymd()),
            tags: tag_presenters,
            created_at: todo.created_at.to_rfc3339(),
            updated_at: todo.updated_at.to_rfc3339(),
        }
    }
}
