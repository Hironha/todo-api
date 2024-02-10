use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::find::FindTodoController;
use crate::adapters::dtos::todo::find::{FindTodoRequest, ParseError};
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
use crate::application::dtos::todo::find::FindTodoError;
use crate::application::use_cases::todo::find::FindTodoUseCase;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct FindPathParams {
    id: Option<String>,
}

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<FindPathParams>,
) -> impl IntoResponse {
    let req = FindTodoRequest { id: path.id };

    tracing::info!("Find todo request: {req:?}");

    let presenter = JsonTodoPresenter::new();
    let interactor = FindTodoUseCase::new(state.todo_repository);
    let controller = FindTodoController::new(interactor, presenter);
    let output = match controller.run(req).await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("Find todo error: {err:?}");
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
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(find_err) = error.downcast_ref::<FindTodoError>() {
        return match find_err {
            FindTodoError::NotFound => {
                let api_error = ApiError::new("NotFound", find_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            FindTodoError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
