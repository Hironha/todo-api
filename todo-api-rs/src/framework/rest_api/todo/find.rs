use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::{adapters::todo::find_input::FindTodoInput, application::functions::todo};

#[derive(Deserialize)]
pub(super) struct GetTodoPath {
    id: Option<String>,
}

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<GetTodoPath>,
) -> impl IntoResponse {
    let input = FindTodoInput { id: path.id };

    println!("GET TODO -> input: {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(message) => return (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
    };

    let ctx = todo::GetContext {
        store: state.todo_store,
    };
    let result = todo::get_todo(ctx, &payload).await;

    let todo = match result {
        Ok(todo) => todo,
        Err(message) => return (StatusCode::NOT_FOUND, message).into_response(),
    };

    (StatusCode::OK, Json(todo)).into_response()
}
