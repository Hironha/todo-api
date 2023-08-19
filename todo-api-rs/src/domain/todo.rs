use serde::Serialize;

use super::types::{Date, DateTime, Id};

#[derive(Clone, Debug, Serialize)]
pub struct Todo {
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename(serialize = "todoAt", deserialize = "todoAt"))]
    pub todo_at: Option<Date>,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(serialize = "updatedAt", deserialize = "updatedAt"))]
    pub updated_at: DateTime,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
