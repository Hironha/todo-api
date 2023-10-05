use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTagInput(pub Id);

#[derive(Clone, Debug)]
pub struct DeleteTagOutput(Result<(), DeleteTagError>);

impl DeleteTagOutput {
    pub const fn ok() -> Self {
        Self(Ok(()))
    }

    pub const fn err(err: DeleteTagError) -> Self {
        Self(Err(err))
    }

    pub fn into_result(self) -> Result<(), DeleteTagError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeleteTagError {
    NotFound,
    Internal,
}
