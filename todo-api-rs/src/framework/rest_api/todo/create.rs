use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use super::TodoState;
use crate::adapters::todo::create_input::CreateTodoInput;
use crate::application::functions::todo;

pub async fn create_todo(
    State(state): State<TodoState>,
    Json(input): Json<CreateTodoInput>,
) -> impl IntoResponse {
    println!("CREATE TODO -> input: {input:?}");

    let payload = match input.into_payload() {
        Ok(payload) => payload,
        Err(message) => return (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
    };

    let mut ctx = todo::CreateContext {
        store: state.todo_store,
    };
    let result = todo::create_todo(&mut ctx, payload).await;

    match result {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(message) => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
    }
}
