use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::update::UpdateTodoController;
use crate::adapters::dtos::todo::update::UpdateRequest;
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
use crate::application::use_cases::todo::update::UpdateTodoUseCase;

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
    status: Option<String>,
}

pub(super) async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<UpdatePathParams>,
    Json(body): Json<UpdateBody>,
) -> impl IntoResponse {
    let req = UpdateRequest {
        id: path.id,
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
        status: body.status,
    };

    tracing::info!("Update todo request: {req:?}");

    let presenter = JsonTodoPresenter::new();
    let interactor = UpdateTodoUseCase::new(state.todo_repository);
    let controller = UpdateTodoController::new(interactor, presenter);
    if let Err(err) = controller.run(req).await {
        if let Some(src) = err.src() {
            tracing::error!("Update todo internal error: {src}");
        } else {
            tracing::error!("Update todo error: {err:?}");
        }

        let status = match StatusCode::from_u16(err.status()) {
            Ok(status) => status,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(err.content)).into_response()
    } else {
        (StatusCode::OK).into_response()
    }
}
