use crate::application::dtos::tag::list::{ListTagError, ListTagOutput};
use crate::application::repositories::tag::list::{List, ListError};

pub async fn list_tags<S: List>(ctx: ListTagContext<S>) -> ListTagOutput {
    match ctx.store.list().await {
        Ok(tags) => ListTagOutput::ok(tags),
        Err(err) => ListTagOutput::err(match err {
            ListError::Internal => ListTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct ListTagContext<S: List> {
    store: S,
}

impl<S: List> ListTagContext<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }
}
