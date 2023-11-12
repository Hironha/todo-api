use std::error::Error;
use std::fmt;

use crate::domain::entities::tag::{Description, Name};

#[derive(Clone, Debug)]
pub struct CreateTagInput {
    pub name: Name,
    pub description: Description,
}

#[derive(Debug)]
pub enum CreateTagError {
    Repository(Box<dyn Error>),
}

impl fmt::Display for CreateTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for CreateTagError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
