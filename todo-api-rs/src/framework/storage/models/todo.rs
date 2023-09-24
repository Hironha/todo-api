use sqlx::types::time::{Date, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::FromRow;

use crate::application::repositories::todo::create::CreatePayload;
use crate::domain::todo::{Title, Todo, Description};

#[derive(Clone, Debug, FromRow)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<CreatePayload> for TodoModel {
    fn from(payload: CreatePayload) -> Self {
        let current_date_time = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            title: payload.title.value(),
            description: payload.description.value(),
            todo_at: payload.todo_at.map(|at| at.value()),
            created_at: current_date_time,
            updated_at: current_date_time,
        }
    }
}

impl TodoModel {
    pub fn into_entity(self) -> Todo {
        Todo {
            id: self.id.into(),
            title: Title::new(self.title).unwrap(),
            description: Description::new(self.description).unwrap(),
            todo_at: self.todo_at.map(|at| at.into()),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}

impl PartialEq for TodoModel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
