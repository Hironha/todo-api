use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTagInput(pub Id);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeleteTagError {
    NotFound,
    Internal,
}
