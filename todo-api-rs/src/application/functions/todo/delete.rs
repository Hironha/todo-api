use crate::application::dtos::todo::delete::{DeleteTodoError, DeleteTodoInput};
use crate::application::repositories::todo::delete::{Delete, DeleteError};

pub async fn delete_todo<Repo: Delete>(
    ctx: DeleteTodoContext<'_, Repo>,
    DeleteTodoInput(id): DeleteTodoInput,
) -> Result<(), DeleteTodoError> {
    ctx.repository.delete(id).await.map_err(|err| match err {
        DeleteError::NotFound => DeleteTodoError::NotFound,
        DeleteError::Internal(err) => DeleteTodoError::Repository(err),
    })
}

#[derive(Clone, Debug)]
pub struct DeleteTodoContext<'a, Repo: Delete> {
    repository: &'a Repo,
}

impl<'a, S: Delete> DeleteTodoContext<'a, S> {
    pub const fn new(repository: &'a S) -> Self {
        Self { repository }
    }
}
