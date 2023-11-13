use std::error::Error;
use std::fmt;

use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError, TodoEntity};
use crate::domain::types::Date;
use crate::framework::storage::models::todo::TodoModel;

pub fn map_todo_model_to_entity(model: TodoModel) -> Result<TodoEntity, MapTodoModelError> {
    let title = Title::new(model.title).map_err(MapTodoModelError::Title)?;
    let description =
        Description::new(model.description).map_err(MapTodoModelError::Description)?;

    Ok(TodoEntity {
        id: model.id.into(),
        title,
        description,
        done: model.done,
        todo_at: model.todo_at.map(Date::from),
        created_at: model.created_at.into(),
        updated_at: model.updated_at.into(),
    })
}

#[derive(Debug)]
pub enum MapTodoModelError {
    Title(TitleError),
    Description(DescriptionError),
}

impl fmt::Display for MapTodoModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title(err) => write!(f, "todo model title incompatible with entity: {err}"),
            Self::Description(err) => {
                write!(f, "todo model description incompatible with entity: {err}")
            }
        }
    }
}

impl Error for MapTodoModelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Title(err) => Some(err),
            Self::Description(err) => Some(err),
        }
    }
}
