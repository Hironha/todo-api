use serde::Serialize;

use crate::domain::entities::tag::TagEntity;

#[derive(Clone, Debug, Serialize)]
pub struct TagPresenter {
    id: String,
    name: String,
    description: Option<String>,
    #[serde(rename(serialize = "createdAt"))]
    created_at: String,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: String,
}

impl From<TagEntity> for TagPresenter {
    fn from(tag: TagEntity) -> Self {
        Self {
            id: tag.id.to_string(),
            name: tag.name.into_string(),
            description: tag.description.map(|d| d.into_string()),
            created_at: tag.created_at.to_rfc3339(),
            updated_at: tag.updated_at.to_rfc3339(),
        }
    }
}
