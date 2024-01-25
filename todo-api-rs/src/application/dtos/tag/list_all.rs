use std::error;

use thiserror::Error;

use crate::domain::entities::tag::TagEntity;

#[derive(Clone, Debug)]
pub struct ListAllTagsOutput {
    pub items: Vec<TagEntity>,
    pub count: usize,
}

#[derive(Debug, Error)]
pub enum ListAllTagsError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
