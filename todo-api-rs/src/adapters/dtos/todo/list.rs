use serde::Serialize;

use crate::adapters::views::todo::TodoView;

#[derive(Debug)]
pub struct Output(Result<OutputData, RunError>);
impl Output {
    pub const fn ok(data: OutputData) -> Self {
        Self(Ok(data))
    }

    pub const fn err(error: RunError) -> Self {
        Self(Err(error))
    }

    pub fn value(self) -> Result<OutputData, RunError> {
        self.0
    }
}

#[derive(Debug, Serialize)]
pub struct OutputData {
    pub count: usize,
    pub items: Vec<TodoView>,
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    Internal,
}
