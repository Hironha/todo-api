use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::bind_tags::BindTagsController;
use crate::adapters::dtos::todo::bind_tags::{BindTagsRequest, ParseError, RunError};
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
    let controller = BindTagsController::new(state.todo_repository);

    if let Err(err) = controller.run(input).await {
        let (status, error) = config_error_response(err);
        return (status, Json(error)).into_response();
    }

    (StatusCode::OK).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(err) => {
            let field = match err {
                ParseError::EmptyTodo | ParseError::InvalidTodo => "todoId",
                ParseError::InvalidTag(_) => "tagsId",
            };
            let details = ValidationError::new(field, err.to_string());
            let error = ApiError::new("BTD-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::TodoNotFound => {
            let error = ApiError::new("BTD-002", "Todo not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::TagNotFound => {
            let error = ApiError::new("BTD-003", "Tag not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("BTD-004", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
