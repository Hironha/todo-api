use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TagState;
use crate::adapters::controllers::tag::find::FindController;
use crate::adapters::dtos::tag::find::{ParseError, RawInput, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};

pub(super) async fn find_tag(
    State(state): State<TagState>,
    Path(path): Path<Value>,
) -> impl IntoResponse {
    tracing::info!("find tag path: {path:?}");

    let input = RawInput {
        id: path.as_str().map(|id| id.to_string()),
    };

    let controller = FindController::new(state.tag_store);
    let output = match controller.run(input).await.into_result() {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::CREATED, Json(output)).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(e) => {
            let field = match e {
                ParseError::EmptyId | ParseError::InvalidId => "id",
            };
            let details = ValidationError::new(field, e.description());
            let error = ApiError::new("FTG-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("FTG-002", "Tag not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("FTG-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
