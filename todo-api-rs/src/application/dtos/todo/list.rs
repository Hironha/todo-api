use std::num::NonZeroU32;

use crate::domain::entities::todo::TodoEntity;

#[derive(Clone, Debug)]
pub struct ListTodoInput {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
}

#[derive(Clone, Debug)]
pub struct ListTodoOutput(Result<Vec<TodoEntity>, ListTodoError>);

impl ListTodoOutput {
    pub const fn ok(todos: Vec<TodoEntity>) -> Self {
        Self(Ok(todos))
    }

    pub const fn err(error: ListTodoError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<Vec<TodoEntity>, ListTodoError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListTodoError {
    Internal,
}
