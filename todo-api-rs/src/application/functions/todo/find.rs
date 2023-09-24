use crate::application::repositories::todo::find::{Find, FindError};
use crate::domain::entities::todo::Todo;
use crate::domain::types::Id;

pub async fn find_todo<T: Find>(
    ctx: FindTodoContext<T>,
    FindTodoInput(id): FindTodoInput,
) -> Result<Todo, FindTodoError> {
    ctx.store.find(id).await.map_err(|e| match e {
        FindError::NotFound => FindTodoError::NotFound,
        FindError::Internal => FindTodoError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct FindTodoInput(Id);
impl FindTodoInput {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
}

pub struct FindTodoContext<T: Find> {
    pub store: T,
}

#[derive(Clone, Debug)]
pub enum FindTodoError {
    NotFound,
    Internal,
}
