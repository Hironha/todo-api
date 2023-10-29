use crate::domain::entities::tag::{Description, Name};

#[derive(Clone, Debug)]
pub struct CreateTagInput {
    pub name: Name,
    pub description: Description,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTagError {
    Internal,
}
