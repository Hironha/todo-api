use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::delete::DeleteController;
use crate::adapters::dtos::tag::delete::{DeleteRequest, ParseError};
use crate::application::dtos::tag::delete::DeleteTagError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct DeletePathParams {
    id: Option<String>,
}

pub(super) async fn delete_tag(
    State(state): State<TagState>,
    Path(path): Path<DeletePathParams>,
) -> impl IntoResponse {
    tracing::info!("Delete tag path: {path:?}");

    let input = DeleteRequest { id: path.id };
    let controller = DeleteController::new(state.tag_repository);

    if let Err(err) = controller.run(input).await {
        tracing::error!("Delete tag error: {err:?}");
        let (status_code, message) = config_error_response(err);
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
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

    if let Some(delete_err) = error.downcast_ref::<DeleteTagError>() {
        return match delete_err {
            DeleteTagError::NotFound => {
                let api_error = ApiError::new("NotFound", delete_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            DeleteTagError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
