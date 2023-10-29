use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeleteTodoInput(pub Id);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeleteTodoError {
    NotFound,
    Internal,
}
