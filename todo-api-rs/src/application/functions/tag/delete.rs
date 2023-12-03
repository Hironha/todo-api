use crate::application::dtos::tag::delete::{DeleteTagError, DeleteTagInput};
use crate::application::repositories::tag::{DeleteError, TagRepository};

pub async fn delete_tag<T>(
    ctx: DeleteTagContext<'_, T>,
    DeleteTagInput(id): DeleteTagInput,
) -> Result<(), DeleteTagError>
where
    T: TagRepository,
{
    ctx.tag_repository
        .delete(id)
        .await
        .map_err(|err| match err {
            DeleteError::NotFound => DeleteTagError::NotFound,
            DeleteError::Internal(err) => DeleteTagError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct DeleteTagContext<'a, T>
where
    T: TagRepository,
{
    pub tag_repository: &'a T,
}
