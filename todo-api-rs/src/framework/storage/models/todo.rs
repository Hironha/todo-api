use std::error;

use serde::Deserialize;
use sqlx::types::time::{Date as TimeDate, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

use crate::domain::entities::todo::{
    Description, InitProps, Status as EntityStatus, Title, TodoEntity,
};
use crate::domain::types::Date;

#[derive(Clone, Debug, FromRow, Deserialize)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<TimeDate>,
    pub status: Status,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TodoModel {
    pub fn try_into_entity(self) -> Result<TodoEntity, Box<dyn error::Error>> {
        let title = Title::new(self.title)?;
        let description = self.description.map(Description::new).transpose()?;

        let entity = TodoEntity::init(InitProps {
            id: self.id.into(),
            title,
            description,
            status: self.status.into_entity(),
            todo_at: self.todo_at.map(Date::from),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        });

        Ok(entity)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Type, Deserialize)]
#[sqlx(type_name = "todo_status", rename_all = "snake_case")]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl From<EntityStatus> for Status {
    fn from(value: EntityStatus) -> Self {
        match value {
            EntityStatus::Todo => Status::Todo,
            EntityStatus::InProgress => Status::InProgress,
            EntityStatus::Done => Status::Done,
        }
    }
}

impl From<&EntityStatus> for Status {
    fn from(value: &EntityStatus) -> Self {
        match value {
            EntityStatus::Todo => Status::Todo,
            EntityStatus::InProgress => Status::InProgress,
            EntityStatus::Done => Status::Done,
        }
    }
}

impl Status {
    pub fn into_entity(self) -> EntityStatus {
        match self {
            Self::Todo => EntityStatus::Todo,
            Self::InProgress => EntityStatus::InProgress,
            Self::Done => EntityStatus::Done,
        }
    }
}
