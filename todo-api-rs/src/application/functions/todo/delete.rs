use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::domain::types::Id;

pub async fn delete_todo<S: Delete>(
    ctx: DeleteTodoContext<S>,
    DeleteTodoInput(id): DeleteTodoInput,
) -> Result<(), DeleteTodoError> {
    ctx.store.delete(id).await.map_err(|e| match e {
        DeleteError::NotFound => DeleteTodoError::NotFound,
        DeleteError::Internal => DeleteTodoError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct DeleteTodoInput(Id);
impl DeleteTodoInput {
    pub fn new(id: Id) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug)]
pub struct DeleteTodoContext<T: Delete> {
    pub store: T,
}

#[derive(Clone, Debug)]
pub enum DeleteTodoError {
    NotFound,
    Internal,
}
