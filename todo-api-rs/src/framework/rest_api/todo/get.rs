use super::TodoState;
use crate::{adapters::todo::get_input::GetTodoInput, application::functions::todo};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_todo(
    State(state): State<TodoState>,
    Query(input): Query<GetTodoInput>,
) -> impl IntoResponse {
    let payload = match input.into_payload() {
        Ok(payload) => payload,
        Err(message) => return (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
    };

    let ctx = todo::GetContext {
        store: state.todo_store,
        payload,
    };
    let result = todo::get_todo(ctx).await;

    let todo = match result {
        Ok(todo) => todo,
        Err(message) => return (StatusCode::NOT_FOUND, message).into_response(),
    };

    (StatusCode::OK, Json(todo)).into_response()
}
