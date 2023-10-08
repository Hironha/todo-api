use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::create::CreateController;
use crate::adapters::dtos::todo::create::{ParseError, RawInput, RunError};
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct CreateBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

pub(super) async fn create_todo(
    State(state): State<TodoState>,
    Json(body): Json<CreateBody>,
) -> impl IntoResponse {
    tracing::info!("create todo body: {body:?}");

    let input = RawInput {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
    };
    let controller = CreateController::new(state.todo_store);

    let output = match controller.run(input).await.into_result() {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::CREATED, Json(output)).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(e) => {
            let field = match e {
                ParseError::EmptyTitle | ParseError::InvalidTitle(_) => "title",
                ParseError::InvalidDescription(_) => "description",
                ParseError::TodoAt => "todoAt",
            };
            let details = ValidationError::new(field, e.description());
            let error = ApiError::new("CTD-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::Internal => {
            let error = ApiError::new("CTD-002", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
