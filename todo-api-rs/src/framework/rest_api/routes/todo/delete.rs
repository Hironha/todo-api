use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::delete::DeleteTodoController;
use crate::adapters::dtos::todo::delete::{DeleteTodoRequest, ParseError};
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
use crate::application::dtos::todo::delete::DeleteTodoError;
use crate::application::use_cases::todo::delete::DeleteTodoUseCase;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct DeletePathParams {
    id: Option<String>,
}

pub(super) async fn delete_todo(
    State(state): State<TodoState>,
    Path(path): Path<DeletePathParams>,
) -> impl IntoResponse {
    let req = DeleteTodoRequest { id: path.id };

    tracing::info!("Delete todo request {req:?}");

    let presenter = JsonTodoPresenter::new();
    let interactor = DeleteTodoUseCase::new(state.todo_repository);
    let controller = DeleteTodoController::new(interactor, presenter);
    if let Err(err) = controller.run(req).await {
        tracing::error!("Delete todo error: {err:?}");
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

    if let Some(delete_err) = error.downcast_ref::<DeleteTodoError>() {
        return match delete_err {
            DeleteTodoError::NotFound => {
                let api_error = ApiError::new("NotFound", delete_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            DeleteTodoError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
