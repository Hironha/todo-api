use serde::Serialize;

use crate::adapters::presenters::tag::TagPresenter;

#[derive(Clone, Debug, Serialize)]
pub struct TagList {
    pub items: Vec<TagPresenter>,
    pub count: u64,
}

#[derive(Clone, Debug)]
pub enum RunError {
    Internal,
}
