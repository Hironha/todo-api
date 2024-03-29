mod create;
mod delete;
mod find;
mod list;
mod update;

use axum::extract::FromRef;
use axum::routing::{get, post};
use axum::Router;
use sqlx::{Pool, Postgres};

use crate::framework::storage::repositories::todo::PgTodoRepository;

use create::create_todo;
use delete::delete_todo;
use find::find_todo;
use list::list_todo;
use update::update_todo;

pub fn create_router(pool: Pool<Postgres>) -> Router {
    let state = TodoState {
        todo_repository: PgTodoRepository::new(pool.clone()),
    };

    Router::new()
        .route("/todos", post(create_todo).get(list_todo))
        .route(
            "/todos/:id",
            get(find_todo).delete(delete_todo).put(update_todo),
        )
        .with_state(state)
}

#[derive(FromRef, Clone)]
struct TodoState {
    todo_repository: PgTodoRepository,
}
