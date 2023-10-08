use crate::application::dtos::tag::delete::{DeleteTagError, DeleteTagInput, DeleteTagOutput};
use crate::application::repositories::tag::delete::{Delete, DeleteError};

pub async fn delete_tag<S: Delete>(
    ctx: DeleteTagContext<'_, S>,
    DeleteTagInput(id): DeleteTagInput,
) -> DeleteTagOutput {
    match ctx.store.delete(id).await {
        Ok(_) => DeleteTagOutput::ok(),
        Err(err) => DeleteTagOutput::err(match err {
            DeleteError::NotFound => DeleteTagError::NotFound,
            DeleteError::Internal => DeleteTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct DeleteTagContext<'a, S: Delete> {
    store: &'a S,
}

impl<'a, S: Delete> DeleteTagContext<'a, S> {
    pub const fn new(store: &'a S) -> Self {
        Self { store }
    }
}
