use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTagInput(pub Id);

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
