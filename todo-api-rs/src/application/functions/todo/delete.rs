use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::domain::types::Id;

pub async fn delete_todo<S: Delete>(ctx: DeleteContext<S>, id: Id) -> Result<(), DeleteTodoError> {
    ctx.store.delete(id).await.map_err(|e| match e {
        DeleteError::NotFound => DeleteTodoError::NotFound,
        DeleteError::Internal => DeleteTodoError::Internal,
    })
}

pub struct DeleteContext<T: Delete> {
    pub store: T,
}

pub enum DeleteTodoError {
    NotFound,
    Internal,
}
