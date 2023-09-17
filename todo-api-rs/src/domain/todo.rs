use super::types::{Date, DateTime, Id};

#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
