use std::error::Error;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::bind_tags::BindTodoTagsController;
use crate::adapters::dtos::todo::bind_tags::{BindTodoTagsRequest, ParseError};
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
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
    let req = BindTodoTagsRequest {
        tags_id: body.tags_id,
        todo_id: path.id,
    };

    tracing::info!("Bind todo tags request: {req:?}");

    let presenter = JsonTodoPresenter::new();
    let controller = BindTodoTagsController::new(state.todo_repository, presenter);
    if let Err(err) = controller.run(req).await {
        tracing::error!("Bind todo tags error: {err:?}");
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
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(bind_err) = error.downcast_ref::<BindTodoTagsError>() {
        return match bind_err {
            BindTodoTagsError::TodoNotFound => {
                let api_error = ApiError::new("TodoNotFound", bind_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            BindTodoTagsError::TagNotFound(_) => {
                let api_error = ApiError::new("TagNotFound", bind_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            BindTodoTagsError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
