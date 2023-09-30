use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use super::TodoState;
use crate::adapters::controllers::todo::list::ListController;
use crate::adapters::dtos::todo::list::RunError;
use crate::framework::rest_api::errors::ApiError;

pub(super) async fn list_todos(State(state): State<TodoState>) -> impl IntoResponse {
    let controller = ListController::new(state.todo_store);

    let output = match controller.run().await.into_result() {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<()>) {
    match error {
        RunError::Internal => {
            let error = ApiError::new("LTD-001", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
