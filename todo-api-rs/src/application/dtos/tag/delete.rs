use std::error::Error;
use std::fmt;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTagInput(pub Id);

#[derive(Debug)]
pub enum DeleteTagError {
    NotFound,
    Repository(Box<dyn Error>),
}

impl fmt::Display for DeleteTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag could not be found"),
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for DeleteTagError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
