use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;
use thiserror::Error;

use crate::domain::entities::tag::{Description, DescriptionError, Name, NameError, TagEntity};

#[derive(Clone, Debug, FromRow, Deserialize)]
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

#[derive(Debug, Error)]
pub enum TagModelEntityError {
    #[error("Tag model name incompatible with entity: {0}")]
    Name(#[source] NameError),
    #[error("Tag model description incompatible entity: {0}")]
    Description(#[source] DescriptionError),
}
