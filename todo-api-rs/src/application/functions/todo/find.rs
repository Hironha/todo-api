use crate::application::dto::todo::find::{FindTodoError, FindTodoInput, FindTodoOutput};
use crate::application::repositories::todo::find::{Find, FindError};

pub async fn find_todo<T: Find>(ctx: FindTodoContext<T>, input: FindTodoInput) -> FindTodoOutput {
    match ctx.store.find(input.into_id()).await {
        Ok(todo) => FindTodoOutput::ok(todo),
        Err(err) => FindTodoOutput::err(match err {
            FindError::NotFound => FindTodoError::NotFound,
            FindError::Internal => FindTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct FindTodoContext<T: Find> {
    pub store: T,
}
