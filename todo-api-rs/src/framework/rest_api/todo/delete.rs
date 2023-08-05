use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use super::TodoState;
use crate::{adapters::todo::delete_input::DeleteTodoInput, application::functions::todo};

pub async fn delete_todo(
    State(state): State<TodoState>,
    Path(input): Path<DeleteTodoInput>,
) -> impl IntoResponse {
    println!("DELETE TODO -> input {input:?}");

    let payload = match input.into_payload() {
        Ok(payload) => payload,
        Err(message) => return (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
    };

    let mut ctx = todo::DeleteContext {
        store: state.todo_store,
    };
    let result = todo::delete_todo(&mut ctx, &payload).await;

    let todo = match result {
        Ok(todo) => todo,
        Err(message) => return (StatusCode::NOT_FOUND, message).into_response(),
    };

    println!("DELETE TODO -> deleted: {todo:?}");

    (StatusCode::NO_CONTENT).into_response()
}
