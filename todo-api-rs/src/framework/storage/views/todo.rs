use std::error::Error;

use serde::Deserialize;
use sqlx::types::{time, Json, Uuid};
use sqlx::FromRow;

use crate::domain::entities::tag::TagEntity;
use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::framework::storage::models::tag::TagModel;
use crate::framework::storage::models::todo::Status as TodoModelStatus;

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
    pub tags: Json<Vec<TagModel>>,
}

impl TodoWithTagsView {
    pub fn try_into_entity(self) -> Result<TodoEntity, Box<dyn Error>> {
        let title = Title::new(self.title)?;
        let description = self.description.map(Description::new).transpose()?;
        let tags = self
            .tags
            .0
            .into_iter()
            .map(TagModel::try_into_entity)
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
