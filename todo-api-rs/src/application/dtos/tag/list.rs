use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListTagError {
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
