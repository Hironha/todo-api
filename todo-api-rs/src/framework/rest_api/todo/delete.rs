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
        Err(error) => {
            let message = error.validation_error();
            return (StatusCode::BAD_REQUEST, Json(message)).into_response();
        }
    };

    let ctx = todo::DeleteContext {
        store: state.todo_store,
    };

    if let Err(error) = todo::delete_todo(&ctx, &payload).await {
        let (status_code, message) = error.api_error();
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
}

impl ParseError {
    fn validation_error(&self) -> ApiError<ValidationError> {
        let field = match self {
            Self::Id => "id",
        };
        ApiError::from(ValidationError::new(field.into(), self.description()))
    }
}

impl todo::DeleteError {
    fn api_error(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ApiError {
                    code: "DTD-001".to_string(),
                    message: "Todo not found".to_string(),
                },
            ),
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "DTD-002".to_string(),
                    message: "Internal server error".to_string(),
                },
            ),
        }
    }
}
