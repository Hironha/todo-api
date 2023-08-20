use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use super::TodoState;
use crate::{
    adapters::todo::delete::{DeleteInput, ParseError},
    application::functions::todo,
};

impl ParseError {
    fn message(&self) -> String {
        match self {
            Self::Id => format!("id: {}", self.description()),
        }
    }
}

#[derive(Deserialize)]
pub(super) struct DeleteTodoPath {
    id: Option<String>,
}

pub(super) async fn delete_todo(
    State(state): State<TodoState>,
    Path(path): Path<DeleteTodoPath>,
) -> impl IntoResponse {
    let input = DeleteInput { id: path.id };

    println!("DELETE TODO -> input {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(err) => return (StatusCode::UNPROCESSABLE_ENTITY, err.message()).into_response(),
    };

    let ctx = todo::DeleteContext {
        store: state.todo_store,
    };
    let result = todo::delete_todo(&ctx, &payload).await;

    if let Err(message) = result {
        (StatusCode::NOT_FOUND, message).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
}
