use sqlx::FromRow;
use uuid::Uuid;

use crate::{application::functions::todo::CreatePayload, domain::todo::Todo};

#[derive(Clone, Debug, FromRow)]
pub struct TodoModel {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<CreatePayload> for TodoModel {
    fn from(value: CreatePayload) -> Self {
        TodoModel {
            id: Uuid::new_v4().to_string(),
            title: value.title,
            description: value.description,
            todo_at: value.todo_at,
            created_at: "created_at".to_string(),
            updated_at: "updated_at".to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Todo> for TodoModel {
    fn into(self) -> Todo {
        Todo {
            id: self.id,
            title: self.title,
            description: self.description,
            todo_at: self.todo_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
