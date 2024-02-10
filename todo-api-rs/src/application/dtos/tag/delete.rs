use std::error;

use thiserror::Error;

use crate::domain::types::Id;

pub type DeleteTagInput = Id;
pub type DeleteTagOutput = Result<(), DeleteTagError>;

#[derive(Debug, Error)]
pub enum DeleteTagError {
    #[error("Tag could not be found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
}
