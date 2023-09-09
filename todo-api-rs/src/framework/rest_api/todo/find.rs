use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::find::{FindController, RunError};
use crate::adapters::dtos::todo::find::{InputSchema, ParseError};
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

    let controller = FindController::new(input_schema, state.todo_store);
    let output = match controller.run().await {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = get_error_response_config(err);
            return (status_code, Json(message)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn get_error_response_config(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Validation(e) => {
            let validation_error = ValidationError::new(get_parse_error_field(&e), e.description());
            let error = ApiError::new("FTD-001", "Invalid input").with_details(validation_error);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("FTD-002", "Todo not found");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
        RunError::Internal => {
            let error = ApiError::new("FTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}

fn get_parse_error_field(error: &ParseError) -> &'static str {
    match error {
        ParseError::Id => "id",
    }
}
