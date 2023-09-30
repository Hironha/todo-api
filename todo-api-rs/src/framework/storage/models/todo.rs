use sqlx::types::time::{Date, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::FromRow;

use crate::application::repositories::todo::create::CreatePayload;
use crate::domain::entities::todo::{Description, Title, TodoEntity};

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
            title: payload.title.into_string(),
            description: payload.description.into_opt_string(),
            todo_at: payload.todo_at.map(|at| at.into_date()),
            created_at: current_date_time,
            updated_at: current_date_time,
        }
    }
}

impl TodoModel {
    pub fn into_entity(self) -> TodoEntity {
        TodoEntity {
            id: self.id.into(),
            title: Title::new(self.title).unwrap(),
            description: Description::new(self.description).unwrap(),
            todo_at: self.todo_at.map(|at| at.into()),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}
