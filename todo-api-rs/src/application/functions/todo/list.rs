use crate::application::dtos::todo::list::{ListTodoError, ListTodoOutput};
use crate::application::repositories::todo::list::{List, ListError};

pub async fn list_todo<T: List>(ctx: ListContext<T>) -> ListTodoOutput {
    match ctx.store.list().await {
        Ok(todos) => ListTodoOutput::ok(todos),
        Err(err) => ListTodoOutput::err(match err {
            ListError::Internal => ListTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct ListContext<T: List> {
    pub store: T,
}
