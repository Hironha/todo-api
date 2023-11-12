use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::delete::DeleteController;
use crate::adapters::dtos::tag::delete::{DeleteRequest, ParseError, RunError};
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
    tracing::info!("delete tag path: {path:?}");

    let input = DeleteRequest { id: path.id };
    let controller = DeleteController::new(state.tag_repository);

    if let Err(err) = controller.run(input).await {
        let (status_code, message) = config_error_response(&err);
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
}

fn config_error_response(error: &RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(parse_err) => {
            let field = match parse_err {
                ParseError::EmptyId | ParseError::InvalidId => "id",
            };
            let details = ValidationError::new(field, parse_err.to_string());
            let api_error = ApiError::new("DTG-001", error.to_string()).with_details(details);
            (StatusCode::BAD_REQUEST, api_error)
        }
        RunError::Deleting(delete_err) => match &delete_err {
            DeleteTagError::NotFound => {
                let api_error = ApiError::new("DTG-002", delete_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            DeleteTagError::Repository(repository_err) => {
                tracing::error!("delete tag repository error: {repository_err}");
                let api_error = ApiError::new("DTG-003", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        },
    }
}
