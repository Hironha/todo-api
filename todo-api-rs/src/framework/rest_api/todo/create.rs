use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::create_input::CreateTodoInput;
use crate::application::functions::todo;

#[derive(Deserialize)]
pub(super) struct CreateTodoBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

pub(super) async fn create_todo(
    State(state): State<TodoState>,
    Json(body): Json<CreateTodoBody>,
) -> impl IntoResponse {
    let input = CreateTodoInput {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
    };

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
