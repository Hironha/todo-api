use serde::Serialize;

use crate::domain::entities::todo::TodoEntity;

/// Presentable format of `Todo` entity, *i.e.* the most appropriate format
/// to be used by framework layer
#[derive(Clone, Debug, Serialize)]
pub struct TodoView {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    /// `None` or `Date` stringified with UTC YYYY-MM-DD format
    #[serde(rename(serialize = "todoAt"))]
    pub todo_at: Option<String>,
    /// `Date` stringified with `RFC-3339` format
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
    /// `Date` stringified with `RFC-3339` format
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
}

impl From<TodoEntity> for TodoView {
    fn from(todo: TodoEntity) -> Self {
        TodoView {
            id: todo.id.to_string(),
            title: todo.title.into_string(),
            description: todo.description.into_opt_string(),
            todo_at: todo.todo_at.map(|at| at.to_ymd()),
            created_at: todo.created_at.to_rfc3339(),
            updated_at: todo.updated_at.to_rfc3339(),
        }
    }
}
