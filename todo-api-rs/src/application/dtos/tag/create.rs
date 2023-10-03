use crate::domain::entities::tag::{Description, Name, TagEntity};

#[derive(Clone, Debug)]
pub struct CreateTagInput {
    pub name: Name,
    pub description: Description,
}

#[derive(Clone, Debug)]
pub struct CreateTagOutput(Result<TagEntity, CreateTagError>);

impl CreateTagOutput {
    pub const fn ok(tag: TagEntity) -> Self {
        Self(Ok(tag))
    }

    pub const fn err(error: CreateTagError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<TagEntity, CreateTagError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTagError {
    Internal,
}
