use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct UpdateTagInput {
    pub id: Id,
    pub name: Name,
    pub description: Description,
}

#[derive(Clone, Debug)]
pub struct UpdateTagOutput(Result<TagEntity, UpdateTagError>);

impl UpdateTagOutput {
    pub const fn ok(tag: TagEntity) -> Self {
        Self(Ok(tag))
    }

    pub const fn err(err: UpdateTagError) -> Self {
        Self(Err(err))
    }

    pub fn into_result(self) -> Result<TagEntity, UpdateTagError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateTagError {
    NotFound,
    Internal,
}
