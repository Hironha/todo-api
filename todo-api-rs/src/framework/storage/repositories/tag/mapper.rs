use std::error::Error;
use std::fmt;

use crate::domain::entities::tag::{Description, DescriptionError, Name, NameError, TagEntity};
use crate::framework::storage::models::tag::TagModel;

pub fn map_tag_model_to_entity(model: TagModel) -> Result<TagEntity, MapTagModelError> {
    let name = Name::new(model.name).map_err(MapTagModelError::Name)?;
    let description = Description::new(model.description).map_err(MapTagModelError::Description)?;

    Ok(TagEntity {
        id: model.id.into(),
        name,
        description,
        created_at: model.created_at.into(),
        updated_at: model.updated_at.into(),
    })
}

#[derive(Debug)]
pub enum MapTagModelError {
    Name(NameError),
    Description(DescriptionError),
}

impl fmt::Display for MapTagModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(err) => write!(f, "tag model name incompatible with entity: {err}"),
            Self::Description(err) => {
                write!(f, "tag model description incompatible with entity: {err}")
            }
        }
    }
}

impl Error for MapTagModelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Name(err) => Some(err),
            Self::Description(err) => Some(err),
        }
    }
}
