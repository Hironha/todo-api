use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::delete::DeleteTodoController;
use crate::adapters::dtos::todo::delete::DeleteRequest;
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
use crate::application::use_cases::todo::delete::DeleteTodoUseCase;

#[derive(Clone, Debug, Deserialize)]
pub(super) struct DeletePathParams {
    id: Option<String>,
}

pub(super) async fn delete_todo(
    State(state): State<TodoState>,
    Path(path): Path<DeletePathParams>,
) -> impl IntoResponse {
    let req = DeleteRequest { id: path.id };

    tracing::info!("Delete todo request {req:?}");

    let presenter = JsonTodoPresenter::new();
    let interactor = DeleteTodoUseCase::new(state.todo_repository);
    let controller = DeleteTodoController::new(interactor, presenter);
    if let Err(err) = controller.run(req).await {
        if let Some(src) = err.src() {
            tracing::error!("Delete todo internal error: {src}");
        } else {
            tracing::error!("Delete todo error: {err:?}");
        }

        let status = match StatusCode::from_u16(err.status()) {
            Ok(status) => status,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        return (status, Json(err.content)).into_response();
    }

    (StatusCode::NO_CONTENT).into_response()
}
