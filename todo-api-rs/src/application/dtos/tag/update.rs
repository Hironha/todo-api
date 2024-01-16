use std::error;

use thiserror::Error;

use crate::domain::entities::tag::{Description, Name};
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct UpdateTagInput {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(Debug, Error)]
pub enum UpdateTagError {
    #[error("Tag could not be found")]
    NotFound,
    #[error("Tag with name {0} already exists")]
    DuplicatedName(Name),
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
