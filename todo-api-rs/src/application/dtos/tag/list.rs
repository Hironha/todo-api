use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ListTagError {
    Repository(Box<dyn Error>),
}

impl fmt::Display for ListTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repository(err) => err.fmt(f),
        }
    }
}

impl Error for ListTagError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Repository(err) => Some(err.as_ref()),
        }
    }
}
