use crate::application::dtos::tag::list::{ListTagError, ListTagOutput};
use crate::application::repositories::tag::list::{List, ListError};

pub async fn list_tag<S: List>(ctx: ListTagContext<'_, S>) -> ListTagOutput {
    match ctx.store.list().await {
        Ok(tags) => ListTagOutput::ok(tags),
        Err(err) => ListTagOutput::err(match err {
            ListError::Internal => ListTagError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct ListTagContext<'a, S: List> {
    store: &'a S,
}

impl<'a, S: List> ListTagContext<'a, S> {
    pub const fn new(store: &'a S) -> Self {
        Self { store }
    }
}
