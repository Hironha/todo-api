use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::update::UpdateController;
use crate::adapters::dtos::todo::update::{ParseError, RawInput, RunError};
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
    done: Option<bool>,
}

pub(super) async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<UpdatePathParams>,
    Json(body): Json<UpdateBody>,
) -> impl IntoResponse {
    tracing::info!("update todo path input {path:?}");
    tracing::info!("update todo body {body:?}");

    let input = RawInput {
        id: path.id,
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
        done: body.done,
    };
    let controller = UpdateController::new(state.todo_store);

    let output = match controller.run(input).await.into_result() {
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
        RunError::Validation(e) => {
            let field = match e {
                ParseError::EmptyId | ParseError::InvalidId => "id",
                ParseError::EmptyTitle | ParseError::InvalidTitle(_) => "title",
                ParseError::InvalidDescription(_) => "description",
                ParseError::TodoAt => "todoAt",
                ParseError::EmptyDone => "done"
            };
            let details = ValidationError::new(field, e.description());
            let error = ApiError::new("UTD-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("UTD-002", "Todo not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("UTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
