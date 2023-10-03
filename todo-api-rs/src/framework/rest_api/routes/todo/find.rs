use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TodoState;
use crate::adapters::controllers::todo::find::FindController;
use crate::adapters::dtos::todo::find::{RawInput, ParseError, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<Value>,
) -> impl IntoResponse {
    tracing::info!("find todo path input {path:?}");

    let input_schema = RawInput {
        id: path.as_str().map(|id| id.to_string()),
    };
    let controller = FindController::new(state.todo_store);

    let output = match controller.run(input_schema).await.into_result() {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = config_error_response(err);
            return (status_code, Json(message)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Validation(e) => {
            let field = match e {
                ParseError::EmptyId | ParseError::InvalidId => "id",
            };
            let details = ValidationError::new(field, e.description());
            let error = ApiError::new("FTD-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("FTD-002", "Todo not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("FTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
