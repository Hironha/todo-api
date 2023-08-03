use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename(serialize = "todoAt", deserialize = "todoAt"))]
    pub todo_at: Option<String>,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: String,
    #[serde(rename(serialize = "updatedAt", deserialize = "updatedAt"))]
    pub updated_at: String,
}
