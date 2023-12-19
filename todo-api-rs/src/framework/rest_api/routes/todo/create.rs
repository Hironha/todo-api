use axum::extract::State;
use axum::http::{header, StatusCode};
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
    status: Option<String>,
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
        status: body.status,
    };

    let controller = CreateController::new(state.todo_repository);
    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(&err);
            return (status, Json(error)).into_response();
        }
    };

    let mut headers = header::HeaderMap::new();
    if let Ok(location) = format!("/todos/{}", output.id).parse::<header::HeaderValue>() {
        headers.insert(header::LOCATION, location);
    }

    (StatusCode::CREATED, headers, Json(output)).into_response()
}

fn config_error_response(run_err: &RunError) -> (StatusCode, ApiError<ValidationError>) {
    match run_err {
        RunError::Parsing(parse_err) => {
            let field = match parse_err {
                ParseError::EmptyTitle | ParseError::InvalidTitle(_) => "title",
                ParseError::InvalidDescription(_) => "description",
                ParseError::InvalidTodoAt => "todoAt",
                ParseError::EmptyStatus | ParseError::InvalidStatus(_) => "status",
            };
            let details = ValidationError::new(field, parse_err.to_string());
            let api_error = ApiError::new("CTD-001", run_err.to_string()).with_details(details);
            (StatusCode::BAD_REQUEST, api_error)
        }
        RunError::Creating(create_err) => match create_err {
            CreateTodoError::DuplicatedTitle(..) => {
                let api_error = ApiError::new("CTD-002", create_err.to_string());
                (StatusCode::CONFLICT, api_error)
            }
            CreateTodoError::Repository(repo_err) => {
                tracing::error!("create todo repository error: {repo_err}");
                let api_error = ApiError::new("CTD-003", run_err.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        },
    }
}
