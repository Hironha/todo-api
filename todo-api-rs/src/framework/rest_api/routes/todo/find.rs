use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::find::FindController;
use crate::adapters::dtos::todo::find::{FindRequest, ParseError, RunError};
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
    tracing::info!("find todo path input {path:?}");

    let input = FindRequest { id: path.id };
    let controller = FindController::new(state.todo_repository);

    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = config_error_response(&err);
            return (status_code, Json(message)).into_response();
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
            let api_error = ApiError::new("FTD-001", error.to_string()).with_details(details);
            (StatusCode::BAD_REQUEST, api_error)
        }
        RunError::Finding(find_err) => match find_err {
            FindTodoError::NotFound => {
                let api_error = ApiError::new("FTD-002", find_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            FindTodoError::Repository(repository_err) => {
                tracing::error!("find todo repository error: {repository_err}");
                let api_error = ApiError::new("FTD-003", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        },
    }
}
