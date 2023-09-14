mod create;
mod delete;
mod find;
mod list;
mod update;

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

use create::create_todo;
use delete::delete_todo;
use find::find_todo;
use list::list_todos;
use update::update_todo;

use crate::framework::storage::TodoStore;

#[derive(Clone, FromRef)]
pub struct TodoState {
    todo_store: TodoStore,
}

pub fn create_router(pool: Pool<Postgres>) -> Router {
    let state = TodoState {
        todo_store: TodoStore::new(pool),
    };

    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .route(
            "/todos/:id",
            get(find_todo).delete(delete_todo).put(update_todo),
        )
        .with_state(state)
}
