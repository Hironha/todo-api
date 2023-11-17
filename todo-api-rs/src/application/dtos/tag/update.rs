use std::error::Error;
use std::fmt;

use crate::domain::entities::tag::{Description, Name};
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct UpdateTagInput {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(Debug)]
pub enum UpdateTagError {
    NotFound,
    Repository(Box<dyn Error>),
}

impl fmt::Display for UpdateTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag could not be found"),
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for UpdateTagError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
