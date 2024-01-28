use std::error::Error;

use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::create::CreateTodoController;
use crate::adapters::dtos::todo::create::{CreateTodoRequest, ParseError};
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
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
    let req = CreateTodoRequest {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
        status: body.status,
    };

    tracing::info!("Create todo request: {req:?}");

    let presenter = JsonTodoPresenter::new();
    let controller = CreateTodoController::new(state.todo_repository, presenter);
    let output = match controller.run(req).await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("Create todo error: {err:?}");
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    let mut headers = header::HeaderMap::new();
    if let Ok(location) = format!("/todos/{}", output.id).parse::<header::HeaderValue>() {
        headers.insert(header::LOCATION, location);
    }

    (StatusCode::CREATED, headers, Json(output)).into_response()
}

fn config_error_response(error: Box<dyn Error>) -> (StatusCode, ApiError<ValidationError>) {
    if let Some(parse_err) = error.downcast_ref::<ParseError>() {
        let field = get_parse_error_field(parse_err);
        let details = ValidationError::new(field, parse_err.to_string());
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(create_err) = error.downcast_ref::<CreateTodoError>() {
        return match create_err {
            CreateTodoError::DuplicatedTitle(..) => {
                let api_error = ApiError::new("DuplicatedTitle", create_err.to_string());
                (StatusCode::CONFLICT, api_error)
            }
            CreateTodoError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}

fn get_parse_error_field(err: &ParseError) -> &str {
    match err {
        ParseError::Title(_) => "title",
        ParseError::Description(_) => "description",
        ParseError::TodoAt => "todoAt",
        ParseError::Status(_) => "status",
    }
}
