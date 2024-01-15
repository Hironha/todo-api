use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::find::FindController;
use crate::adapters::dtos::tag::find::{FindRequest, ParseError};
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
            tracing::error!("find tag error: {err:?}");
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: Box<dyn Error>) -> (StatusCode, ApiError<ValidationError>) {
    if let Some(parse_err) = error.downcast_ref::<ParseError>() {
        let field = match parse_err {
            ParseError::EmptyId | ParseError::InvalidId => "id",
        };
        let details = ValidationError::new(field, parse_err.to_string());
        let api_error = ApiError::new("FTG-001", error.to_string()).with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(find_err) = error.downcast_ref::<FindTagError>() {
        return match find_err {
            FindTagError::NotFound => {
                let api_error = ApiError::new("FTG-002", find_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            FindTagError::Repository(..) => {
                let api_error = ApiError::internal("FTG-003");
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    let default_err = ApiError::new("FTG-004", error.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, default_err)
}
