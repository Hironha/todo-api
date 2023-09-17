use crate::application::repositories::todo::list::{List, ListError};
use crate::domain::todo::Todo;

pub async fn list_todo<T: List>(ctx: ListContext<T>) -> Result<Vec<Todo>, ListTodoError> {
    ctx.store.list().await.map_err(|e| match e {
        ListError::Internal => ListTodoError::Internal,
    })
}

pub enum ListTodoError {
    Internal,
}

pub struct ListContext<T: List> {
    pub store: T,
}
