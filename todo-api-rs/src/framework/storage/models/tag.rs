use std::error;

use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;

use crate::domain::entities::tag::{Description, Name, TagEntity};

#[derive(Clone, Debug, FromRow, Deserialize)]
pub struct TagModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TagModel {
    pub fn try_into_entity(self) -> Result<TagEntity, Box<dyn error::Error>> {
        let name = Name::new(self.name)?;
        let description = self.description.map(Description::new).transpose()?;

        Ok(TagEntity {
            id: self.id.into(),
            name,
            description,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        })
    }
}
