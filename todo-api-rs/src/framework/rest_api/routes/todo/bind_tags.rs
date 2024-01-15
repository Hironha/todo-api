use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::bind_tags::BindTagsController;
use crate::adapters::dtos::todo::bind_tags::{BindTagsRequest, ParseError};
use crate::application::dtos::todo::bind_tags::BindTodoTagsError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct BindTagsBody {
    #[serde(rename(deserialize = "tagsId"))]
    pub(super) tags_id: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct BindTagsPathParams {
    pub(super) id: Option<String>,
}

pub(super) async fn bind_todo_tags(
    State(state): State<TodoState>,
    Path(path): Path<BindTagsPathParams>,
    Json(body): Json<BindTagsBody>,
) -> impl IntoResponse {
    let req = BindTagsRequest {
        tags_id: body.tags_id,
        todo_id: path.id,
    };

    tracing::info!("bind todo tags request: {req:?}");

    let controller = BindTagsController::new(state.todo_repository, state.tag_repository);
    if let Err(err) = controller.run(req).await {
        tracing::error!("bind todo tags error: {err}");
        let (status, error) = config_error_response(err);
        return (status, Json(error)).into_response();
    }

    (StatusCode::OK).into_response()
}

fn config_error_response(error: Box<dyn Error>) -> (StatusCode, ApiError<ValidationError>) {
    if let Some(parse_err) = error.downcast_ref::<ParseError>() {
        let field = match parse_err {
            ParseError::EmptyTodo | ParseError::InvalidTodo => "todoId",
            ParseError::InvalidTag(_) => "tagsId",
        };
        let details = ValidationError::new(field, parse_err.to_string());
        let api_error = ApiError::new("BTD-001", "invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(bind_err) = error.downcast_ref::<BindTodoTagsError>() {
        return match bind_err {
            BindTodoTagsError::TodoNotFound => {
                let api_error = ApiError::new("BTD-002", bind_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            BindTodoTagsError::TagNotFound(_) => {
                let api_error = ApiError::new("BTD-003", bind_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            BindTodoTagsError::Repository(..) => {
                let api_error = ApiError::internal("BTD-004");
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    let default_err = ApiError::new("BTD-005", error.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, default_err)
}
