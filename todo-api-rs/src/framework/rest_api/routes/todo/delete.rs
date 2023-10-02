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
    tracing::info!("delete todo path input {path:?}");

    let input_schema = RawInput {
        id: path.as_str().map(|id| id.to_string()),
    };
    let controller = DeleteController::new(state.todo_store);

    if let Err(err) = controller.run(input_schema).await.into_result() {
        let (status_code, message) = config_error_response(err);
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(e) => {
            let field = match e {
                ParseError::EmptyId | ParseError::InvalidId => "id",
            };
            let details = ValidationError::new(field, e.description());
            let error = ApiError::new("DTD-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::TodoNotFound => {
            let error = ApiError::new("DTD-002", "Todo not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("DTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
