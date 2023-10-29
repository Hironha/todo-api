use crate::application::dtos::tag::delete::{DeleteTagError, DeleteTagInput};
use crate::application::repositories::tag::delete::{Delete, DeleteError};

pub async fn delete_tag<Repo: Delete>(
    ctx: DeleteTagContext<'_, Repo>,
    DeleteTagInput(id): DeleteTagInput,
) -> Result<(), DeleteTagError> {
    ctx.repository.delete(id).await.map_err(|err| match err {
        DeleteError::NotFound => DeleteTagError::NotFound,
        DeleteError::Internal => DeleteTagError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct DeleteTagContext<'a, Repo: Delete> {
    repository: &'a Repo,
}

impl<'a, Repo: Delete> DeleteTagContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
