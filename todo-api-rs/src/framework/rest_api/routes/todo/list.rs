use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::list::ListTodosController;
use crate::adapters::dtos::todo::list::ListRequest;
use crate::adapters::presenters::json::todo::JsonTodoPresenter;
use crate::application::use_cases::todo::list::ListTodosUseCase;

#[derive(Clone, Debug, Deserialize)]
pub(super) struct QueryParams {
    page: Option<u32>,
    #[serde(rename(deserialize = "perPage"))]
    per_page: Option<u32>,
    title: Option<String>,
}

pub(super) async fn list_todo(
    State(state): State<TodoState>,
    Query(query): Query<QueryParams>,
) -> impl IntoResponse {
    let req = ListRequest {
        page: query.page,
        per_page: query.per_page,
        title: query.title,
    };

    tracing::info!("List todos request: {req:?}");

    let presenter = JsonTodoPresenter::new();
    let interactor = ListTodosUseCase::new(state.todo_repository);
    let controller = ListTodosController::new(interactor, presenter);
    let output = match controller.run(req).await {
        Ok(output) => output,
        Err(err) => {
            if let Some(src) = err.src() {
                tracing::error!("List todos internal error: {src}");
            } else {
                tracing::error!("List todos error: {err:?}");
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
