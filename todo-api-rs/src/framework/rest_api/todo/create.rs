use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::create::{CreateInput, ParseError};
use crate::application::functions::todo;

impl ParseError {
    fn message(&self) -> String {
        match self {
            Self::Title => format!("title: {}", self.description()),
            Self::TodoAt => format!("todoAt: {}", self.description()),
        }
    }
}

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
    let input = CreateInput {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
    };

    println!("CREATE TODO -> input: {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(err) => return (StatusCode::UNPROCESSABLE_ENTITY, err.message()).into_response(),
    };

    let ctx = todo::CreateContext {
        store: state.todo_store,
    };
    let result = todo::create_todo(&ctx, payload).await;

    match result {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(message) => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
    }
}
