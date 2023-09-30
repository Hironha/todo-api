use crate::domain::entities::todo::{Description, Title, Todo};
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug)]
pub struct UpdateTodoInput {
    pub id: Id,
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

#[derive(Clone, Debug)]
pub struct UpdateTodoOutput(Result<Todo, UpdateTodoError>);

impl UpdateTodoOutput {
    pub const fn ok(todo: Todo) -> Self {
        Self(Ok(todo))
    }

    pub const fn err(error: UpdateTodoError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<Todo, UpdateTodoError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateTodoError {
    NotFound,
    Internal,
}
