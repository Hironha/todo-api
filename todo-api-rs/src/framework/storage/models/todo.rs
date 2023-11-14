use sqlx::types::time::{Date as TimeDate, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::FromRow;

use crate::domain::entities::tag::TagEntity;
use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::domain::types::Date;

#[derive(Clone, Debug, FromRow)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<TimeDate>,
    pub done: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TodoModel {
    pub fn into_entity(self, tags: Vec<TagEntity>) -> TodoEntity {
        TodoEntity {
            id: self.id.into(),
            title: Title::new_unchecked(self.title),
            description: Description::new_unchecked(self.description),
            done: self.done,
            todo_at: self.todo_at.map(Date::from),
            tags,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}
