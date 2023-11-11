use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::create::CreateController;
use crate::adapters::dtos::todo::create::{CreateRequest, ParseError, RunError};
use crate::application::dtos::todo::create::CreateTodoError;
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

    let input = CreateRequest {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
    };
    let controller = CreateController::new(state.todo_repository);

    let output = match controller.run(input).await {
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
            let details = ValidationError::new(field, e.to_string());
            let error = ApiError::new("CTD-001", "invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::Creating(err) => match err {
            CreateTodoError::Repository(err) => {
                tracing::error!("create todo repository error: {err}");
                let error = ApiError::new("CTD-002", "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
        },
    }
}
