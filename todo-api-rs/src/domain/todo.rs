use serde::Serialize;

use super::types::{SerializableDate, SerializableDateTime};

#[derive(Clone, Debug, Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename(serialize = "todoAt", deserialize = "todoAt"))]
    pub todo_at: Option<SerializableDate>,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: SerializableDateTime,
    #[serde(rename(serialize = "updatedAt", deserialize = "updatedAt"))]
    pub updated_at: SerializableDateTime,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
