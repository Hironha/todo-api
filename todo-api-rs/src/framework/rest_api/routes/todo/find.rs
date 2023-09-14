use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TodoState;
use crate::adapters::controllers::todo::find::FindController;
use crate::adapters::dtos::todo::find::{InputSchema, ParseError, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};
use crate::framework::rest_api::extractors::StringExtractor;

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<Value>,
) -> impl IntoResponse {
    println!("GET TODO -> path: {path:#?}");

    let input_schema = match extract_input_schema(path) {
        Ok(input) => input,
        Err(err) => return (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response(),
    };
    let controller = FindController::new(input_schema, state.todo_store);

    let output = match controller.run().await.value() {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = config_error_response(err);
            return (status_code, Json(message)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn extract_input_schema(mut path: Value) -> Result<InputSchema, ApiError<ValidationError>> {
    let id = StringExtractor::optional("id").extract(&mut path)?;

    Ok(InputSchema { id })
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
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
