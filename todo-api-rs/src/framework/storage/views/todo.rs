use std::error::Error;

use serde::Deserialize;
use sqlx::types::{time, Json, Uuid};
use sqlx::FromRow;

use crate::domain::entities::tag::{Description as TagDescription, Name, TagEntity};
use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::domain::types::DateTime;
use crate::framework::storage::models::todo::TodoStatus as TodoModelStatus;

#[derive(Clone, Debug, Deserialize, FromRow)]
// TODO: maybe in a later refactor of domain layer, rename this to
// `TodoAggregateView``
pub struct TodoWithTagsView {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoModelStatus,
    pub todo_at: Option<time::Date>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
    pub tags: Json<Vec<JsonTodoTag>>,
}

impl TodoWithTagsView {
    pub fn try_into_entity(self) -> Result<TodoEntity, Box<dyn Error>> {
        let title = Title::new(self.title)?;
        let description = self.description.map(Description::new).transpose()?;
        let tags = self
            .tags
            .0
            .into_iter()
            .map(JsonTodoTag::try_into_entity)
            .collect::<Result<Vec<TagEntity>, Box<dyn Error>>>()?;

        Ok(TodoEntity {
            id: self.id.into(),
            title,
            description,
            status: self.status.into_entity(),
            todo_at: self.todo_at.map(|at| at.into()),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            tags,
        })
    }
}

#[derive(Clone, Debug, Deserialize, FromRow)]
pub struct JsonTodoTag {
    id: Uuid,
    name: String,
    description: Option<String>,
    // TODO: find a way to deserialize directly to time::OffsetDateTime
    // so it's possible to just use `TagModel` instead of `JsonTodoTag`
    created_at: String,
    updated_at: String,
}

impl JsonTodoTag {
    fn try_into_entity(self) -> Result<TagEntity, Box<dyn Error>> {
        let name = Name::new(self.name)?;
        let description = self.description.map(TagDescription::new).transpose()?;
        let created_at = DateTime::parse_str(self.created_at.as_str())?;
        let updated_at = DateTime::parse_str(self.updated_at.as_str())?;

        Ok(TagEntity {
            id: self.id.into(),
            name,
            description,
            created_at,
            updated_at,
        })
    }
}
