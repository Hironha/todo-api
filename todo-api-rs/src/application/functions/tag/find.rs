use crate::application::dtos::tag::find::{FindTagError, FindTagInput, FindTagOutput};
use crate::application::repositories::tag::find::{Find, FindError};

pub async fn find_tag<S: Find>(ctx: FindTagContext<S>, input: FindTagInput) -> FindTagOutput {
    match ctx.store.find(input.into_id()).await {
        Ok(todo) => FindTagOutput::ok(todo),
        Err(err) => FindTagOutput::err(match err {
            FindError::NotFound => FindTagError::NotFound,
            FindError::Internal => FindTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct FindTagContext<S: Find> {
    pub store: S,
}
