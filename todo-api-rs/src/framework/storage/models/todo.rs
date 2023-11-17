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
    /// Panics if not compatible with `TodoEntity`
    pub fn into_entity(self, tags: Vec<TagEntity>) -> TodoEntity {
        let title = Title::new(self.title).expect("todo model title not compatible with entity");
        let description = Description::new(self.description)
            .expect("todo model description not compatible with entity");

        TodoEntity {
            id: self.id.into(),
            title,
            description,
            done: self.done,
            todo_at: self.todo_at.map(Date::from),
            tags,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}
