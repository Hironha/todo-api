use thiserror::Error;

use sqlx::types::time::{Date as TimeDate, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

use crate::domain::entities::tag::TagEntity;
use crate::domain::entities::todo::{
    Description, DescriptionError, Title, TitleError, TodoEntity, TodoStatus as TodoEntityStatus,
};
use crate::domain::types::Date;

#[derive(Clone, Debug, FromRow)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<TimeDate>,
    pub status: TodoStatus,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TodoModel {
    pub fn try_into_entity(self, tags: Vec<TagEntity>) -> Result<TodoEntity, TodoModelEntityError> {
        let title = Title::new(self.title).map_err(TodoModelEntityError::Title)?;
        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(TodoModelEntityError::Description)?;

        Ok(TodoEntity {
            id: self.id.into(),
            title,
            description,
            status: self.status.into_entity(),
            todo_at: self.todo_at.map(Date::from),
            tags,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Type)]
#[sqlx(type_name = "todo_status", rename_all = "snake_case")]
pub enum TodoStatus {
    Todo,
    InProgress,
    Done,
}

impl From<TodoEntityStatus> for TodoStatus {
    fn from(value: TodoEntityStatus) -> Self {
        match value {
            TodoEntityStatus::Todo => TodoStatus::Todo,
            TodoEntityStatus::InProgress => TodoStatus::InProgress,
            TodoEntityStatus::Done => TodoStatus::Done,
        }
    }
}

impl TodoStatus {
    fn into_entity(self) -> TodoEntityStatus {
        match self {
            Self::Todo => TodoEntityStatus::Todo,
            Self::InProgress => TodoEntityStatus::InProgress,
            Self::Done => TodoEntityStatus::Done,
        }
    }
}

#[derive(Debug, Error)]
pub enum TodoModelEntityError {
    #[error("todo model title incompatible with entity: {0}")]
    Title(#[source] TitleError),
    #[error("todo model description incompatible with entity: {0}")]
    Description(#[source] DescriptionError),
}
