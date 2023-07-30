use crate::adapters::todo::create_input::CreateTodoInput;
use crate::application::functions::todo;

use super::TodoState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn create_todo(
    State(state): State<TodoState>,
    Json(input): Json<CreateTodoInput>,
) -> impl IntoResponse {
    let payload = match input.into_payload() {
        Ok(payload) => payload,
        Err(message) => return (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
    };
    let ctx = todo::CreateContext {
        creator: state.todo_creator,
    };

    let result = todo::create_todo(ctx, payload).await;
    match result {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(message) => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
    }
}
