use crate::adapters::dtos::tag::list::{Output, RunError};
use crate::application::dtos::tag::list::ListTagError;
use crate::application::functions::tag::list::{list_tags, ListTagContext};
use crate::application::repositories::tag::list::List;

#[derive(Clone, Debug)]
pub struct ListController<S: List> {
    store: S,
}

impl<S: List> ListController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self) -> Output {
        let ctx = ListTagContext::new(self.store);

        match list_tags(ctx).await.into_result() {
            Ok(tags) => Output::from_tags(tags),
            Err(err) => Output::err(match err {
                ListTagError::Internal => RunError::Internal,
            }),
        }
    }
}
