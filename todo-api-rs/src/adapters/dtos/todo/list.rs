use serde::Serialize;

use crate::adapters::views::todo::TodoView;
use crate::domain::entities::todo::TodoEntity;

#[derive(Debug)]
pub struct Output(Result<OutputData, RunError>);
impl Output {
    pub const fn err(error: RunError) -> Self {
        Self(Err(error))
    }

    pub fn from_todos(todos: Vec<TodoEntity>) -> Self {
        let data = OutputData {
            count: todos.len(),
            items: todos.into_iter().map(TodoView::from).collect(),
        };

        Self(Ok(data))
    }

    pub fn into_result(self) -> Result<OutputData, RunError> {
        self.0
    }
}

#[derive(Debug, Serialize)]
pub struct OutputData {
    pub count: usize,
    pub items: Vec<TodoView>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    Internal,
}
