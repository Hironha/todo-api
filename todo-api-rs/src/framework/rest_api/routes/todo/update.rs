use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::update::UpdateController;
use crate::adapters::dtos::todo::update::{ParseError, RunError, UpdateRequest};
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

    tracing::info!("update todo request: {req:?}");

    let controller = UpdateController::new(state.todo_repository);
    if let Err(err) = controller.run(req).await {
        tracing::error!("update todo error: {err}");
        let (status_code, message) = config_error_response(&err);
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::OK).into_response()
    }
}

fn config_error_response(error: &RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(parse_err) => {
            let field = get_parse_error_field(parse_err);
            let details = ValidationError::new(field, parse_err.to_string());
            let api_error = ApiError::new("UTD-001", error.to_string()).with_details(details);
            (StatusCode::BAD_REQUEST, api_error)
        }
        RunError::Updating(update_err) => match update_err {
            UpdateTodoError::NotFound => {
                let api_error = ApiError::new("UTD-002", update_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            UpdateTodoError::DuplicatedTitle(..) => {
                let api_error = ApiError::new("UTD-003", update_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            UpdateTodoError::Repository(..) => {
                let api_error = ApiError::internal("UTD-003");
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        },
    }
}

fn get_parse_error_field(err: &ParseError) -> &str {
    match err {
        ParseError::EmptyId | ParseError::InvalidId => "id",
        ParseError::EmptyTitle | ParseError::InvalidTitle(_) => "title",
        ParseError::InvalidDescription(_) => "description",
        ParseError::InvalidTodoAt => "todoAt",
        ParseError::EmptyStatus | ParseError::InvalidStatus(_) => "status",
    }
}
