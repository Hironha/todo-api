use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::find::FindTodoController;
use crate::adapters::dtos::todo::find::FindRequest;
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
use crate::application::use_cases::todo::find::FindTodoUseCase;

#[derive(Clone, Debug, Deserialize)]
pub(super) struct FindPathParams {
    id: Option<String>,
}

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<FindPathParams>,
) -> impl IntoResponse {
    let req = FindRequest { id: path.id };

    tracing::info!("Find todo request: {req:?}");

    let presenter = JsonTodoPresenter::new();
    let interactor = FindTodoUseCase::new(state.todo_repository);
    let controller = FindTodoController::new(interactor, presenter);
    let output = match controller.run(req).await {
        Ok(output) => output,
        Err(err) => {
            if let Some(src) = err.src() {
                tracing::error!("Find todo internal error: {src}");
            } else {
                tracing::error!("Find todo error: {err:?}");
            }

            let status = match StatusCode::from_u16(err.status()) {
                Ok(status) => status,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };

            return (status, Json(err.content)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}
