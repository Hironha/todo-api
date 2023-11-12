use std::error::Error;
use std::fmt;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTagInput(pub Id);

#[derive(Debug)]
pub enum FindTagError {
    NotFound,
    Repository(Box<dyn Error>),
}

impl fmt::Display for FindTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag could not be found"),
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for FindTagError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
