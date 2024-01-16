use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListTagError {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
