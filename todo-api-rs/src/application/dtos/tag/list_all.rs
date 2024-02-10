use std::error;

use thiserror::Error;

use crate::domain::entities::tag::TagEntity;

pub type ListAllTagsOutput = Result<AllTagsList, ListAllTagsError>;

#[derive(Clone, Debug)]
pub struct AllTagsList {
    pub items: Vec<TagEntity>,
    pub count: usize,
}

#[derive(Debug, Error)]
pub enum ListAllTagsError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
