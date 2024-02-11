use std::error;

use serde::Deserialize;
use sqlx::types::time::{Date as TimeDate, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

use crate::domain::entities::todo::{
    Description, Title, TodoEntity, TodoStatus as TodoEntityStatus,
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

        let entity = TodoEntity::builder()
            .id(self.id.into())
            .title(title)
            .status(self.status.into_entity())
            .description(description)
            .todo_at(self.todo_at.map(Date::from))
            .created_at(Some(self.created_at.into()))
            .updated_at(Some(self.updated_at.into()))
            .build();

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

impl From<TodoEntityStatus> for Status {
    fn from(value: TodoEntityStatus) -> Self {
        match value {
            TodoEntityStatus::Todo => Status::Todo,
            TodoEntityStatus::InProgress => Status::InProgress,
            TodoEntityStatus::Done => Status::Done,
        }
    }
}

impl From<&TodoEntityStatus> for Status {
    fn from(value: &TodoEntityStatus) -> Self {
        match value {
            TodoEntityStatus::Todo => Status::Todo,
            TodoEntityStatus::InProgress => Status::InProgress,
            TodoEntityStatus::Done => Status::Done,
        }
    }
}

impl Status {
    pub fn into_entity(self) -> TodoEntityStatus {
        match self {
            Self::Todo => TodoEntityStatus::Todo,
            Self::InProgress => TodoEntityStatus::InProgress,
            Self::Done => TodoEntityStatus::Done,
        }
    }
}
