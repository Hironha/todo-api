use crate::domain::entities::todo::Todo;

pub struct ListTodoOutput(Result<Vec<Todo>, ListTodoError>);

impl ListTodoOutput {
    pub const fn ok(todos: Vec<Todo>) -> Self {
        Self(Ok(todos))
    }

    pub const fn err(error: ListTodoError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<Vec<Todo>, ListTodoError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListTodoError {
    Internal,
}
