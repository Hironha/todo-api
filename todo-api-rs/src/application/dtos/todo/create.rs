use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

pub struct CreateTodoOutput(Result<TodoEntity, CreateTodoError>);

impl CreateTodoOutput {
    pub const fn ok(todo: TodoEntity) -> Self {
        Self(Ok(todo))
    }

    pub const fn err(error: CreateTodoError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<TodoEntity, CreateTodoError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTodoError {
    Internal,
}
