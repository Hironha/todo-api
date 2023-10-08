use crate::application::dtos::tag::find::{FindTagError, FindTagInput, FindTagOutput};
use crate::application::repositories::tag::find::{Find, FindError};

pub async fn find_tag<S: Find>(
    ctx: FindTagContext<'_, S>,
    FindTagInput(id): FindTagInput,
) -> FindTagOutput {
    match ctx.store.find(id).await {
        Ok(todo) => FindTagOutput::ok(todo),
        Err(err) => FindTagOutput::err(match err {
            FindError::NotFound => FindTagError::NotFound,
            FindError::Internal => FindTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct FindTagContext<'a, S: Find> {
    store: &'a S,
}

impl<'a, S: Find> FindTagContext<'a, S> {
    pub const fn new(store: &'a S) -> Self {
        Self { store }
    }
}
