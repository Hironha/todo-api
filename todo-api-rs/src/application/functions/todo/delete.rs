use crate::application::dtos::todo::delete::{DeleteTodoError, DeleteTodoInput, DeleteTodoOutput};
use crate::application::repositories::todo::delete::{Delete, DeleteError};

pub async fn delete_todo<S: Delete>(
    ctx: DeleteTodoContext<S>,
    input: DeleteTodoInput,
) -> DeleteTodoOutput {
    match ctx.store.delete(input.into_id()).await {
        Ok(_) => DeleteTodoOutput::ok(),
        Err(err) => DeleteTodoOutput::err(match err {
            DeleteError::NotFound => DeleteTodoError::NotFound,
            DeleteError::Internal => DeleteTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct DeleteTodoContext<T: Delete> {
    pub store: T,
}
