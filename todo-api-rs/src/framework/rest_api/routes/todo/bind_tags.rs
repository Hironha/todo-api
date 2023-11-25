use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::bind_tags::BindTagsController;
use crate::adapters::dtos::todo::bind_tags::{BindTagsRequest, ParseError, RunError};
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
    tracing::info!("bind todo tags: {body:?}");

    let input = BindTagsRequest {
        tags_id: body.tags_id,
        todo_id: path.id,
    };
    let controller = BindTagsController::new(state.todo_repository, state.tag_repository);

    if let Err(err) = controller.run(input).await {
        let (status, error) = config_error_response(&err);
        return (status, Json(error)).into_response();
    }

    (StatusCode::OK).into_response()
}

fn config_error_response(error: &RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(parse_err) => {
            let field = match parse_err {
                ParseError::EmptyTodo | ParseError::InvalidTodo => "todoId",
                ParseError::InvalidTag(_) => "tagsId",
            };
            let details = ValidationError::new(field, parse_err.to_string());
            let api_error = ApiError::new("BTD-001", error.to_string()).with_details(details);
            (StatusCode::BAD_REQUEST, api_error)
        }
        RunError::Binding(bind_err) => match bind_err {
            BindTodoTagsError::TodoNotFound => {
                let api_error = ApiError::new("BTD-002", bind_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            BindTodoTagsError::TagNotFound(_) => {
                let api_error = ApiError::new("BTD-003", bind_err.to_string());
                (StatusCode::NOT_FOUND, api_error)
            }
            BindTodoTagsError::Repository(repository_err) => {
                tracing::error!("bind todo tags repository error: {repository_err}");

                let api_error = ApiError::new("BTD-004", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        },
    }
}
