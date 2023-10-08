use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::update::UpdateController;
use crate::adapters::dtos::tag::update::{ParseError, RawInput, RunError};
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct UpdatePathParams {
    id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct UpdateBody {
    name: Option<String>,
    description: Option<String>,
}

pub(super) async fn update_tag(
    State(state): State<TagState>,
    Path(path): Path<UpdatePathParams>,
    Json(body): Json<UpdateBody>,
) -> impl IntoResponse {
    tracing::info!("update tag path: {path:?}");
    tracing::info!("update tag body: {body:?}");

    let input = RawInput {
        id: path.id,
        name: body.name,
        description: body.description,
    };
    let controller = UpdateController::new(state.tag_store);

    let output = match controller.run(input).await.into_result() {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(err) => {
            let field = match err {
                ParseError::EmptyId | ParseError::InvalidId => "id",
                ParseError::EmptyName | ParseError::InvalidName(_) => "name",
                ParseError::InvalidDescription(_) => "description",
            };
            let details = ValidationError::new(field, err.description());
            let error = ApiError::new("UTG-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("UTG-002", "Tag not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("UTG-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
