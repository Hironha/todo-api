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
            let (status_code, message) = config_error_response(err);
            return (status_code, Json(message)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(err) => {
            let field = match err {
                ParseError::EmptyId | ParseError::InvalidId => "id",
            };
            let details = ValidationError::new(field, err.to_string());
            let error = ApiError::new("FTD-001", "invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::Finding(err) => match err {
            FindTodoError::NotFound => {
                let error = ApiError::new("FTD-002", err.to_string());
                (StatusCode::NOT_FOUND, error)
            }
            FindTodoError::Repository(err) => {
                tracing::error!("find todo repository error: {err}");
                let error = ApiError::new("FTD-003", "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
        },
    }
}
