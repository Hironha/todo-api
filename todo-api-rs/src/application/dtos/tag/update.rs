use crate::domain::entities::tag::{Description, Name};
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct UpdateTagInput {
    pub id: Id,
    pub name: Name,
    pub description: Description,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateTagError {
    NotFound,
    Internal,
}
