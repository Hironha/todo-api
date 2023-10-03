use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTagInput(Id);

impl FindTagInput {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }

    pub fn into_id(self) -> Id {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct FindTagOutput(Result<TagEntity, FindTagError>);

impl FindTagOutput {
    pub const fn ok(tag: TagEntity) -> Self {
        Self(Ok(tag))
    }

    pub const fn err(error: FindTagError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<TagEntity, FindTagError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindTagError {
    NotFound,
    Internal,
}
