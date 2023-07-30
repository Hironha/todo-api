use super::TodoState;
use crate::core::todo::{list, Todo};
use axum::{extract::State, Json, http::StatusCode};

pub async fn list_todos(State(state): State<TodoState>) -> (StatusCode, Json<Vec<Todo>>) {
    let ctx = list::ListContext {
        lister: state.todo_store,
    };
    let todos = list::list_todos(ctx).await;
    (StatusCode::OK, Json(todos))
}
