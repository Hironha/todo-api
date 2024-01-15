use serde::Serialize;

use crate::adapters::presenters::tag::TagPresenter;

#[derive(Clone, Debug, Serialize)]
pub struct TagsList {
    pub items: Vec<TagPresenter>,
    pub count: u64,
}
