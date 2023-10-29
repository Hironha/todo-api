use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTagInput(pub Id);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindTagError {
    NotFound,
    Internal,
}
