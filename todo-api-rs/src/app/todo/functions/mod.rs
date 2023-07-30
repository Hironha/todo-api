mod create;
mod list;

use super::store::TodoStore;
use axum::{extract::FromRef, routing::post, Router};
use create::create_todo;
use list::list_todos;

#[derive(Clone, FromRef)]
pub struct TodoState {
    todo_store: TodoStore,
}

pub fn create_router() -> Router {
    let state = TodoState {
        todo_store: TodoStore::new(),
    };
    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .with_state(state)
}
