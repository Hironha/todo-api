use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::delete::{DeleteInput, ParseError};
use crate::application::functions::todo;
use crate::framework::rest_api::{ApiError, ValidationError};

impl From<ParseError> for ApiError<ValidationError> {
    fn from(error: ParseError) -> Self {
        let field = match error {
            ParseError::Id => "id",
        };
        Self::from(ValidationError::new(field.into(), error.description()))
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
        Err(err) => {
            let error = Json(ApiError::from(err));
            return (StatusCode::UNPROCESSABLE_ENTITY, error).into_response();
        }
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
