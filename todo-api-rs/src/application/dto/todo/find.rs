use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindTodoInput(Id);

impl FindTodoInput {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }

    pub fn into_id(self) -> Id {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct FindTodoOutput(Result<TodoEntity, FindTodoError>);

impl FindTodoOutput {
    pub const fn ok(todo: TodoEntity) -> Self {
        Self(Ok(todo))
    }

    pub const fn err(error: FindTodoError) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<TodoEntity, FindTodoError> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindTodoError {
    NotFound,
    Internal,
}
