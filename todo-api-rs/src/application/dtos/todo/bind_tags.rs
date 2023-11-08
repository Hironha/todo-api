use std::error::Error;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct BindTodoTagsInput {
    pub todo_id: Id,
    pub tags_id: Option<Vec<Id>>,
}

#[derive(Debug)]
pub enum BindTodoTagsError {
    TodoNotFound,
    TagNotFound,
    Repository(Box<dyn Error>),
}
