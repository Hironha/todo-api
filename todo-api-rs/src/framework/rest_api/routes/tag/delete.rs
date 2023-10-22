use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::delete::DeleteController;
use crate::adapters::dtos::tag::delete::{ParseError, RawInput, RunError};
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct DeletePathParams {
    id: Option<String>,
}

pub(super) async fn delete_tag(
    State(state): State<TagState>,
    Path(path): Path<DeletePathParams>,
) -> impl IntoResponse {
    tracing::info!("delete tag path: {path:?}");

    let input = RawInput { id: path.id };
    let controller = DeleteController::new(state.tag_repository);

    if let Err(err) = controller.run(input).await.into_result() {
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
            let error = ApiError::new("DTG-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("DTG-002", "Tag not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("DTG-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
