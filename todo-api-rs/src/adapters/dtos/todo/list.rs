use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    /// None or Date stringified on Y-M-D format
    #[serde(rename(serialize = "createdAt"))]
    pub todo_at: Option<String>,
    /// Date stringified on `RFC-3339` format
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
    /// Date stringified on `RFC-3339` format
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub count: usize,
    pub items: Vec<Item>,
}
