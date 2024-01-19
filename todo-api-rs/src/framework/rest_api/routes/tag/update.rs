use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::TagController;
use crate::adapters::dtos::tag::update::{ParseError, UpdateRequest};
use crate::application::dtos::tag::update::UpdateTagError;
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
    let input = UpdateRequest {
        id: path.id,
        name: body.name,
        description: body.description,
    };

    tracing::info!("Update tag input: {input:?}");

    let controller = TagController::new(state.tag_repository);

    let output = match controller.update(input).await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("Update tag error: {err:?}");
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
            ParseError::InvalidName(_) => "name",
            ParseError::InvalidDescription(_) => "description",
        };
        let details = ValidationError::new(field, parse_err.to_string());
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(update_err) = error.downcast_ref::<UpdateTagError>() {
        return match update_err {
            UpdateTagError::NotFound => {
                let api_error = ApiError::new("NotFound", update_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            UpdateTagError::DuplicatedName(..) => {
                let api_error = ApiError::new("DuplicatedName", update_err.to_string());
                (StatusCode::CONFLICT, api_error)
            }
            UpdateTagError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
