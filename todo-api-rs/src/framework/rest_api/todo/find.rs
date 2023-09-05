use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::find::{FindController, RunError};
use crate::adapters::dtos::todo::find::{Input, InputSchema, ParseError};
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Deserialize)]
pub(super) struct PathParams {
    id: Option<String>,
}

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let input_schema = InputSchema { id: path.id };

    println!("GET TODO -> input: {input_schema:?}");

    let input = match Input::parse(input_schema) {
        Ok(input) => input,
        Err(err) => {
            let message = err.api_error();
            return (StatusCode::BAD_REQUEST, Json(message)).into_response();
        }
    };

    let controller = FindController::new(state.todo_store);
    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = err.response_parts();
            return (status_code, Json(message)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

impl ParseError {
    fn api_error(&self) -> ApiError<ValidationError> {
        let field = match self {
            Self::Id => "id",
        };
        ApiError::from(ValidationError::new(field.into(), self.description()))
    }
}

impl RunError {
    fn response_parts(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ApiError {
                    code: "FTD-001".to_string(),
                    message: "Todo not found".to_string(),
                },
            ),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "FTD-002".to_string(),
                    message: "Internal server error".to_string(),
                },
            ),
        }
    }
}
