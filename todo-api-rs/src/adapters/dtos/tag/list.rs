use std::error::Error;
use std::fmt;

use serde::Serialize;

use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::list::ListTagError;

#[derive(Clone, Debug, Serialize)]
pub struct TagList {
    pub items: Vec<TagPresenter>,
    pub count: u64,
}

#[derive(Debug)]
pub enum RunError {
    Listing(ListTagError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Listing(_) => write!(f, "failed listing tags"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Listing(err) => Some(err),
        }
    }
}
