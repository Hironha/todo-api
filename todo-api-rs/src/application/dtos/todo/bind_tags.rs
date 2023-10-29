use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct BindTodoTagsInput {
    pub todo_id: Id,
    pub tags_id: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindTodoTagsError {
    TodoNotFound,
    TagNotFound,
    Internal,
}
