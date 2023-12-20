use std::error;

use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTagInput(pub Id);

#[derive(Debug, Error)]
pub enum FindTagError {
    #[error("tag could not be found")]
    NotFound,
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
