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
    #[error("tag could not be found")]
    NotFound,
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
