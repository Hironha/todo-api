use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TagState;
use crate::adapters::controllers::tag::create::CreateController;
use crate::adapters::dtos::tag::create::{ParseError, RawInput, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};

pub(super) async fn create_tag(
    State(state): State<TagState>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    tracing::info!("create tag body: {body:?}");

    let input_schema = extract_input_schema(body);
    let controller = CreateController::new(state.tag_store);

    let output = match controller.run(input_schema).await.into_result() {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::CREATED, Json(output)).into_response()
}

fn extract_input_schema(body: Value) -> RawInput {
    let name = body["name"].as_str().map(|t| t.to_string());
    let description = body["description"].as_str().map(|d| d.to_string());

    RawInput { name, description }
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(e) => {
            let field = match e {
                ParseError::EmptyName | ParseError::InvalidName(_) => "name",
                ParseError::InvalidDescription(_) => "description",
            };
            let details = ValidationError::new(field, e.description());
            let error = ApiError::new("CTG-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::Internal => {
            let error = ApiError::new("CTG-002", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
