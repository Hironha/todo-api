mod create;

use crate::adapters::todo::store::TodoStore;

use axum::{extract::FromRef, routing::post, Router};
use create::create_todo;

#[derive(Clone, FromRef)]
pub struct TodoState {
    todo_store: TodoStore,
}

pub fn create_router() -> Router {
    let state = TodoState {
        todo_store: TodoStore::new(),
    };

    Router::new()
        .route("/todos", post(create_todo))
        .with_state(state)
}
