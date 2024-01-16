use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::update::UpdateController;
use crate::adapters::dtos::todo::update::{ParseError, UpdateRequest};
use crate::application::dtos::todo::update::UpdateTodoError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct UpdatePathParams {
    id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct UpdateBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
    status: Option<String>,
}

pub(super) async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<UpdatePathParams>,
    Json(body): Json<UpdateBody>,
) -> impl IntoResponse {
    let req = UpdateRequest {
        id: path.id,
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
        status: body.status,
    };

    tracing::info!("Update todo request: {req:?}");

    let controller = UpdateController::new(state.todo_repository);
    if let Err(err) = controller.run(req).await {
        tracing::error!("Update todo error: {err:?}");
        let (status_code, message) = config_error_response(err);
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::OK).into_response()
    }
}

fn config_error_response(error: Box<dyn Error>) -> (StatusCode, ApiError<ValidationError>) {
    if let Some(parse_err) = error.downcast_ref::<ParseError>() {
        let field = get_parse_error_field(parse_err);
        let details = ValidationError::new(field, parse_err.to_string());
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(update_err) = error.downcast_ref::<UpdateTodoError>() {
        return match update_err {
            UpdateTodoError::NotFound => {
                let api_error = ApiError::new("NotFound", update_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            UpdateTodoError::DuplicatedTitle(..) => {
                let api_error = ApiError::new("DuplicatedTitle", update_err.to_string());
                (StatusCode::CONFLICT, api_error)
            }
            UpdateTodoError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}

fn get_parse_error_field(err: &ParseError) -> &str {
    match err {
        ParseError::EmptyId | ParseError::InvalidId => "id",
        ParseError::InvalidTitle(_) => "title",
        ParseError::InvalidDescription(_) => "description",
        ParseError::InvalidTodoAt => "todoAt",
        ParseError::InvalidStatus(_) => "status",
    }
}
