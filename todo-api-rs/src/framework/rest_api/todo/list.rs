use crate::application::functions::todo;

use super::TodoState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub(super) async fn list_todos(State(state): State<TodoState>) -> impl IntoResponse {
    let ctx = todo::ListContext {
        store: state.todo_store,
    };

    let todos = match todo::list_todo(&ctx).await {
        Ok(todos) => todos,
        Err(message) => return (StatusCode::BAD_REQUEST, message).into_response(),
    };

    if todos.is_empty() {
        (StatusCode::NO_CONTENT).into_response()
    } else {
        (StatusCode::OK, Json(todos)).into_response()
    }
}
