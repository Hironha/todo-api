use std::error::Error;
use std::fmt;

use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;

use crate::domain::entities::tag::{Description, DescriptionError, Name, NameError, TagEntity};

#[derive(Clone, Debug, FromRow)]
pub struct TagModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TagModel {
    pub fn try_into_entity(self) -> Result<TagEntity, TagModelEntityError> {
        let name = Name::new(self.name).map_err(TagModelEntityError::Name)?;
        let description = self
            .description
            .map(Description::new)
            .transpose()
            .map_err(TagModelEntityError::Description)?;

        Ok(TagEntity {
            id: self.id.into(),
            name,
            description,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        })
    }
}

#[derive(Debug)]
pub enum TagModelEntityError {
    Name(NameError),
    Description(DescriptionError),
}

impl fmt::Display for TagModelEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(err) => write!(f, "tag model name incompatible with entity: {err}"),
            Self::Description(err) => {
                write!(f, "tag model description incompatible with entity: {err}")
            }
        }
    }
}

impl Error for TagModelEntityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Name(err) => Some(err),
            Self::Description(err) => Some(err),
        }
    }
}
