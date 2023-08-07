use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use super::TodoState;
use crate::{adapters::todo::delete_input::DeleteTodoInput, application::functions::todo};

#[derive(Deserialize)]
pub(super) struct DeleteTodoPath {
    id: Option<String>,
}

pub(super) async fn delete_todo(
    State(state): State<TodoState>,
    Path(path): Path<DeleteTodoPath>,
) -> impl IntoResponse {
    let input = DeleteTodoInput { id: path.id };

    println!("DELETE TODO -> input {input:?}");

    let payload = match input.into_payload() {
        Ok(payload) => payload,
        Err(message) => return (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
    };

    let ctx = todo::DeleteContext {
        store: state.todo_store,
    };
    let result = todo::delete_todo(&ctx, &payload).await;

    let todo = match result {
        Ok(todo) => todo,
        Err(message) => return (StatusCode::NOT_FOUND, message).into_response(),
    };

    println!("DELETE TODO -> deleted: {todo:?}");

    (StatusCode::NO_CONTENT).into_response()
}
