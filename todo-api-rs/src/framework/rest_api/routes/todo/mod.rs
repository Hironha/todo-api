mod create;
mod delete;
mod find;
mod list;
mod update;

use axum::extract::FromRef;
use axum::routing::{get, post};
use axum::Router;
use sqlx::{Pool, Postgres};

use crate::framework::storage::repositories::todo::TodoRepository;

use create::create_todo;
use delete::delete_todo;
use find::find_todo;
use list::list_todos;
use update::update_todo;

#[derive(Clone, FromRef)]
pub struct TodoState {
    todo_repository: TodoRepository,
}

pub fn create_router(pool: Pool<Postgres>) -> Router {
    let state = TodoState {
        todo_repository: TodoRepository::new(pool),
    };

    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .route(
            "/todos/:id",
            get(find_todo).delete(delete_todo).put(update_todo),
        )
        .with_state(state)
}
