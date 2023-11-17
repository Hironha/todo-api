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
    /// Panics if not compatible with `TagEntity`
    pub fn into_entity(self) -> TagEntity {
        let name = Name::new(self.name).expect("tag model title incompatible with entity");
        let description = self
            .description
            .map(Description::new)
            .transpose()
            .expect("tag model description incompatible with entity");

        TagEntity {
            id: self.id.into(),
            name,
            description,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}
