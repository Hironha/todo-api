use sqlx::{
    types::{time, Uuid},
    FromRow,
};

use crate::{application::functions::todo::CreatePayload, domain::todo::Todo};

#[derive(Clone, Debug, FromRow)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<time::Date>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl From<CreatePayload> for TodoModel {
    fn from(value: CreatePayload) -> Self {
        TodoModel {
            id: Uuid::new_v4(),
            title: value.title,
            description: value.description,
            todo_at: None,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Todo> for TodoModel {
    fn into(self) -> Todo {
        Todo {
            id: self.id.to_string(),
            title: self.title,
            description: self.description,
            todo_at: None,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}
