use crate::application::dtos::todo::find::{FindTodoError, FindTodoInput, FindTodoOutput};
use crate::application::repositories::todo::find::{Find, FindError};

pub async fn find_todo<S: Find>(
    ctx: FindTodoContext<'_, S>,
    input: FindTodoInput,
) -> FindTodoOutput {
    match ctx.store.find(input.into_id()).await {
        Ok(todo) => FindTodoOutput::ok(todo),
        Err(err) => FindTodoOutput::err(match err {
            FindError::NotFound => FindTodoError::NotFound,
            FindError::Internal => FindTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct FindTodoContext<'a, S: Find> {
    store: &'a S,
}

impl<'a, S: Find> FindTodoContext<'a, S> {
    pub const fn new(store: &'a S) -> Self {
        Self { store }
    }
}
