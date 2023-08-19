use sqlx::{
    types::{
        time::{Date, OffsetDateTime},
        Uuid,
    },
    FromRow,
};

use crate::{application::functions::todo::CreatePayload, domain::todo::Todo};

#[derive(Clone, Debug, FromRow)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TodoModel {
    pub fn from_payload(payload: CreatePayload) -> Self {
        let current_date_time = OffsetDateTime::now_utc();

        TodoModel {
            id: Uuid::new_v4(),
            title: payload.title,
            description: payload.description,
            todo_at: payload.todo_at.map(|at| at.to_date()),
            created_at: current_date_time,
            updated_at: current_date_time,
        }
    }
}

impl TodoModel {
    pub fn into_entity(self) -> Todo {
        Todo {
            id: self.id.to_string(),
            title: self.title,
            description: self.description,
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
