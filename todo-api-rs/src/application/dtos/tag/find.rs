use std::error;

use thiserror::Error;

use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;

pub type FindTagInput = Id;
pub type FindTagOutput = Result<TagEntity, FindTagError>;

#[derive(Debug, Error)]
pub enum FindTagError {
    #[error("Tag could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
