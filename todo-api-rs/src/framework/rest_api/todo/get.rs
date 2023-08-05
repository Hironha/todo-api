use super::TodoState;
use crate::{adapters::todo::get_input::GetTodoInput, application::functions::todo};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_todo(
    State(state): State<TodoState>,
    Path(input): Path<GetTodoInput>,
) -> impl IntoResponse {
    let payload = match input.into_payload() {
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
