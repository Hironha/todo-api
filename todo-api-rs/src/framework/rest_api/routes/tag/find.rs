use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::find::FindTagController;
use crate::adapters::dtos::tag::find::{FindTagRequest, ParseError};
use crate::adapters::presenters::json::tag::JsonTagPresenter;
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
    tracing::info!("Find tag path: {path:?}");

    let input = FindTagRequest { id: path.id };
    let presenter = JsonTagPresenter::new();
    let controller = FindTagController::new(state.tag_repository, presenter);

    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("Find tag error: {err:?}");
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
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(find_err) = error.downcast_ref::<FindTagError>() {
        return match find_err {
            FindTagError::NotFound => {
                let api_error = ApiError::new("NotFound", find_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            FindTagError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
