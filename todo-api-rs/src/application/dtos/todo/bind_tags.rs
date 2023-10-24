use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct BindTodoTagsInput {
    pub todo_id: Id,
    pub tags_id: Vec<Id>,
}

pub struct BindTodoTagsOutput(Result<(), BindTodoTagsError>);

impl BindTodoTagsOutput {
    pub const fn ok() -> Self {
        Self(Ok(()))
    }

    pub const fn err(error: BindTodoTagsError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<(), BindTodoTagsError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindTodoTagsError {
    TodoNotFound,
    TagNotFound,
    Internal,
}
