use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::find::FindController;
use crate::adapters::dtos::tag::find::{FindRequest, ParseError, RunError};
use crate::application::dtos::tag::find::FindTagError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct FindPathParams {
    id: Option<String>,
}

pub(super) async fn find_tag(
    State(state): State<TagState>,
    Path(path): Path<FindPathParams>,
) -> impl IntoResponse {
    tracing::info!("find tag path: {path:?}");

    let input = FindRequest { id: path.id };
    let controller = FindController::new(state.tag_repository);

    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(&err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: &RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(parse_err) => {
            let field = match parse_err {
                ParseError::EmptyId | ParseError::InvalidId => "id",
            };
            let details = ValidationError::new(field, parse_err.to_string());
            let api_error = ApiError::new("FTG-001", error.to_string()).with_details(details);
            (StatusCode::BAD_REQUEST, api_error)
        }
        RunError::Finding(find_err) => match &find_err {
            FindTagError::NotFound => {
                let api_error = ApiError::new("FTG-002", find_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            FindTagError::Repository(repository_err) => {
                tracing::error!("find tag repository error: {repository_err}");
                let api_error = ApiError::new("FTG-003", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        },
    }
}
