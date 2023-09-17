use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TodoState;
use crate::adapters::controllers::todo::update::UpdateController;
use crate::adapters::dtos::todo::update::{InputSchema, ParseError, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};

pub(super) async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<Value>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    tracing::info!("UPDATE TODO -> path: {path:?}");
    tracing::info!("UPDATE TODO -> body: {body:?}");

    let input_schema = extract_input_schema(path, body);
    let controller = UpdateController::new(state.todo_store);

    let output = match controller.run(input_schema,).await.value() {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = config_error_response(err);
            return (status_code, Json(message)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn extract_input_schema(path: Value, body: Value) -> InputSchema {
    let id = path.as_str().map(|id| id.to_string());
    let title = body["title"].as_str().map(|t| t.to_string());
    let description = body["description"].as_str().map(|d| d.to_string());
    let todo_at = body["todoAt"].as_str().map(|at| at.to_string());

    InputSchema {
        id,
        title,
        description,
        todo_at,
    }
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Validation(e) => {
            let validation_error = ValidationError::new(get_parse_error_field(&e), e.description());
            let error = ApiError::new("UTD-001", "Invalid input").with_details(validation_error);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("UTD-002", "Todo not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("UTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}

fn get_parse_error_field(error: &ParseError) -> &'static str {
    match error {
        ParseError::Id => "id",
        ParseError::Title | ParseError::TitleLength => "title",
        ParseError::DescriptionLength => "description",
        ParseError::TodoAt => "todoAt",
    }
}
