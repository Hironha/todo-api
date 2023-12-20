use std::error;

use thiserror::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTagInput(pub Id);

#[derive(Debug, Error)]
pub enum DeleteTagError {
    #[error("tag could not be found")]
    NotFound,
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
