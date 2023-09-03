use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use super::TodoState;
use crate::adapters::controllers::todo::list::{ListController, RunError};
use crate::framework::rest_api::error::ApiError;

pub(super) async fn list_todos(State(state): State<TodoState>) -> impl IntoResponse {
    let controller = ListController::new(state.todo_store);

    let output = match controller.run().await {
        Ok(todos) => todos,
        Err(e) => {
            let (status, error) = e.api_error();
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

impl RunError {
    fn api_error(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "LTD-001".into(),
                    message: "Internal server error".into(),
                },
            ),
        }
    }
}
