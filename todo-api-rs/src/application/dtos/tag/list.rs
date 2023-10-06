use crate::domain::entities::tag::TagEntity;

#[derive(Clone, Debug)]
pub struct ListTagOutput(Result<Vec<TagEntity>, ListTagError>);

impl ListTagOutput {
    pub const fn ok(tags: Vec<TagEntity>) -> Self {
        Self(Ok(tags))
    }

    pub const fn err(err: ListTagError) -> Self {
        Self(Err(err))
    }

    pub fn into_result(self) -> Result<Vec<TagEntity>, ListTagError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListTagError {
    Internal,
}
