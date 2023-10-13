use std::num::NonZeroU32;

use crate::domain::entities::todo::TodoEntity;

#[derive(Clone, Debug)]
pub struct ListTodoInput {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub title: Option<String>
}

#[derive(Clone, Debug)]
pub struct ListTodoOutput(Result<TodoList, ListTodoError>);

impl ListTodoOutput {
    pub const fn ok(data: TodoList) -> Self {
        Self(Ok(data))
    }

    pub const fn err(error: ListTodoError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<TodoList, ListTodoError> {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct TodoList {
    pub count: u64,
    pub items: Vec<TodoEntity>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListTodoError {
    Internal,
}
