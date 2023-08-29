use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::find::{FindInput, ParseError};
use crate::application::functions::todo;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Deserialize)]
pub(super) struct GetTodoPath {
    id: Option<String>,
}

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<GetTodoPath>,
) -> impl IntoResponse {
    let input = FindInput { id: path.id };

    println!("GET TODO -> input: {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(error) => {
            let message = error.validation_error();
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(message)).into_response();
        }
    };

    let ctx = todo::GetContext {
        store: state.todo_store,
    };

    let todo = match todo::find_todo(ctx, &payload).await {
        Ok(todo) => todo,
        Err(error) => {
            let (status_code, message) = error.api_error();
            return (status_code, Json(message)).into_response();
        }
    };

    (StatusCode::OK, Json(todo)).into_response()
}

impl ParseError {
    fn validation_error(&self) -> ApiError<ValidationError> {
        let field = match self {
            Self::Id => "id",
        };
        ApiError::from(ValidationError::new(field.into(), self.description()))
    }
}

impl todo::FindError {
    fn api_error(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ApiError {
                    code: "FTD-001".to_string(),
                    message: "Todo not found".to_string(),
                },
            ),
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "FTD-002".to_string(),
                    message: "Internal server error".to_string(),
                },
            ),
        }
    }
}
