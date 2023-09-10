use serde::Serialize;

use crate::domain::todo::Todo;

/// Presentable format of `Todo` entity, *i.e.* the most appropriate format
/// to be used by framework layer
#[derive(Clone, Debug, Serialize)]
pub struct TodoView {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    /// `None` or `Date` stringified with UTC YYYY-MM-DD format
    #[serde(rename(serialize = "createdAt"))]
    pub todo_at: Option<String>,
    /// `Date` stringified with `RFC-3339` format
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
    /// `Date` stringified with `RFC-3339` format
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
}

impl From<Todo> for TodoView {
    fn from(todo: Todo) -> Self {
        TodoView {
            id: todo.id.as_string(),
            title: todo.title,
            description: todo.description,
            todo_at: todo.todo_at.map(|at| at.ymd()),
            created_at: todo.created_at.rfc3339(),
            updated_at: todo.updated_at.rfc3339(),
        }
    }
}