use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::create::CreateTodoController;
use crate::adapters::dtos::todo::create::CreateRequest;
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
use crate::application::use_cases::todo::create::CreateTodoUseCase;

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
    let req = CreateRequest {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
        status: body.status,
    };

    tracing::info!("Create todo request: {req:?}");

    let presenter = JsonTodoPresenter::new();
    let interactor = CreateTodoUseCase::new(state.todo_repository);
    let controller = CreateTodoController::new(interactor, presenter);
    let output = match controller.run(req).await {
        Ok(output) => output,
        Err(err) => {
            if let Some(src) = err.src() {
                tracing::error!("Create todo internal error: {src}");
            } else {
                tracing::error!("Create todo error: {err:?}");
            }

            let status = match StatusCode::from_u16(err.status()) {
                Ok(status) => status,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (status, Json(err.content)).into_response();
        }
    };

    let mut headers = header::HeaderMap::new();
    if let Ok(location) = format!("/todos/{}", output.id).parse::<header::HeaderValue>() {
        headers.insert(header::LOCATION, location);
    }

    (StatusCode::CREATED, headers, Json(output)).into_response()
}
