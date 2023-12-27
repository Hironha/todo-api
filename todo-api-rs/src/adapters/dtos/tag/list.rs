use serde::Serialize;
use thiserror::Error;

use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::list::ListTagError;

#[derive(Clone, Debug, Serialize)]
pub struct TagsList {
    pub items: Vec<TagPresenter>,
    pub count: u64,
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    Listing(ListTagError),
}
