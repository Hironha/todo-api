mod create;

use crate::adapters::todo::store::TodoStore;

use axum::{extract::FromRef, routing::get, Router};
use create::create_todo;

#[derive(Clone, FromRef)]
pub struct TodoState {
    todo_creator: TodoStore,
}

pub fn create_router() -> Router {
    let state = TodoState {
        todo_creator: TodoStore::new(),
    };

    Router::new()
        .route("/todos", get(create_todo))
        .with_state(state)
}
