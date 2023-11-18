use std::error::Error;
use std::fmt;

use sqlx::types::time::{Date as TimeDate, OffsetDateTime};
use sqlx::types::Uuid;
use sqlx::FromRow;

use crate::domain::entities::tag::TagEntity;
use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError, TodoEntity};
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
            done: self.done,
            todo_at: self.todo_at.map(Date::from),
            tags,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        })
    }
}

#[derive(Debug)]
pub enum TodoModelEntityError {
    Title(TitleError),
    Description(DescriptionError),
}

impl fmt::Display for TodoModelEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title(err) => write!(f, "todo model title incompatible with entity: {err}"),
            Self::Description(err) => {
                write!(f, "todo model description incompatible with entity: {err}")
            }
        }
    }
}

impl Error for TodoModelEntityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Title(err) => Some(err),
            Self::Description(err) => Some(err),
        }
    }
}
