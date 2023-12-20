use std::error;

use thiserror::Error;

use crate::domain::entities::tag::{Description, Name};

#[derive(Clone, Debug)]
pub struct CreateTagInput {
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(Debug, Error)]
pub enum CreateTagError {
    #[error(transparent)]
    Repository(Box<dyn error::Error>),
}
