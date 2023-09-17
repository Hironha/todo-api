use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TodoState;
use crate::adapters::controllers::todo::delete::DeleteController;
use crate::adapters::dtos::todo::delete::{ParseError, RawInput, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};

pub(super) async fn delete_todo(
    State(state): State<TodoState>,
    Path(path): Path<Value>,
) -> impl IntoResponse {
    tracing::info!("DELETE TODO ->> path {path:?}");

    let input_schema = RawInput {
        id: path.as_str().map(|id| id.to_string()),
    };
    let controller = DeleteController::new(state.todo_store);

    if let Err(err) = controller.run(input_schema).await.value() {
        let (status_code, message) = config_error_response(err);
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Validation(e) => {
            let validation_error = ValidationError::new(get_parse_error_field(&e), e.description());
            let error = ApiError::new("DTD-001", "Invalid input").with_details(validation_error);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error: ApiError<_> = ApiError::new("DTD-002", "Todo not found");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
        RunError::Internal => {
            let error: ApiError<ValidationError> =
                ApiError::new("DTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}

fn get_parse_error_field(error: &ParseError) -> &'static str {
    match error {
        ParseError::Id => "id",
    }
}
