use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::find::FindController;
use crate::adapters::dtos::todo::find::{FindRequest, ParseError};
use crate::application::dtos::todo::find::FindTodoError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct FindPathParams {
    id: Option<String>,
}

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<FindPathParams>,
) -> impl IntoResponse {
    let req = FindRequest { id: path.id };

    tracing::info!("find todo request: {req:?}");

    let controller = FindController::new(state.todo_repository);
    let output = match controller.run(req).await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("find todo error: {err}");
            let (status_code, message) = config_error_response(err);
            return (status_code, Json(message)).into_response();
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
        let api_error = ApiError::new("FTD-001", "invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(find_err) = error.downcast_ref::<FindTodoError>() {
        return match find_err {
            FindTodoError::NotFound => {
                let api_error = ApiError::new("FTD-002", find_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            FindTodoError::Repository(..) => {
                let api_error = ApiError::internal("FTD-003");
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        }
    }

    let default_err = ApiError::new("FTD-004", error.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, default_err)
}
