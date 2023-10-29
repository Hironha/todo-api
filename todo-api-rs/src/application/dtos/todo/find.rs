use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTodoInput(pub Id);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindTodoError {
    NotFound,
    Internal,
}
