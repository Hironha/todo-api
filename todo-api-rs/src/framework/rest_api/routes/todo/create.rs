use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TodoState;
use crate::adapters::controllers::todo::create::CreateController;
use crate::adapters::dtos::todo::create::{ParseError, RawInput, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};

pub(super) async fn create_todo(
    State(state): State<TodoState>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    tracing::info!("CREATE TODO ->> body {body:#?}");

    let input_schema = extract_input_schema(body);
    let controller = CreateController::new(state.todo_store);

    let output = match controller.run(input_schema).await.value() {
        Ok(output) => output,
        Err(err) => {
            tracing::info!("CREATE TODO ERROR ->> {err:#?}");
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::CREATED, Json(output)).into_response()
}

fn extract_input_schema(body: Value) -> RawInput {
    let title = body["title"].as_str().map(|t| t.to_string());
    let description = body["description"].as_str().map(|d| d.to_string());
    let todo_at = body["todoAt"].as_str().map(|at| at.to_string());

    RawInput {
        title,
        description,
        todo_at,
    }
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(e) => {
            let field = match e {
                ParseError::EmptyTitle | ParseError::InvalidTitle(_) => "title",
                ParseError::InvalidDescription(_) => "description",
                ParseError::TodoAt => "todoAt",
            };
            let details = ValidationError::new(field, e.description());
            let error = ApiError::new("CTD-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::Internal => {
            let error = ApiError::new("CTD-002", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}