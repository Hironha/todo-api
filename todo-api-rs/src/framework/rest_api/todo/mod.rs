mod create;
mod delete;
mod get;
mod list;
mod update;

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use create::create_todo;
use delete::delete_todo;
use get::get_todo;
use list::list_todos;
use update::update_todo;

use crate::framework::store::TodoStore;

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
        .route(
            "/todos/:id",
            get(get_todo).delete(delete_todo).put(update_todo),
        )
        .with_state(state)
}
