use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FindTagError {
    #[error("Tag could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
