use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeleteTagError {
    #[error("tag could not be found")]
    NotFound,
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
