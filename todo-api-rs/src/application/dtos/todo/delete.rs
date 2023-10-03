use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTodoInput(Id);

impl DeleteTodoInput {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }

    pub fn into_id(self) -> Id {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct DeleteTodoOutput(Result<(), DeleteTodoError>);

impl DeleteTodoOutput {
    pub const fn ok() -> Self {
        Self(Ok(()))
    }

    pub const fn err(error: DeleteTodoError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<(), DeleteTodoError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeleteTodoError {
    NotFound,
    Internal,
}
