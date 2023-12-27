use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FindTagError {
    #[error("tag could not be found")]
    NotFound,
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
