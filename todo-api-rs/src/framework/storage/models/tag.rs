use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;

use crate::domain::entities::tag::{Description, Name, TagEntity};

#[derive(Clone, Debug, FromRow)]
pub struct TagModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TagModel {
    pub fn into_entity(self) -> TagEntity {
        TagEntity {
            id: self.id.into(),
            name: Name::new_unchecked(self.name),
            description: Description::new_unchecked(self.description),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}
