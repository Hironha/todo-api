use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::delete::DeleteController;
use crate::adapters::dtos::todo::delete::{InputSchema, ParseError, RunError};
use crate::framework::rest_api::{ApiError, ValidationError};

#[derive(Deserialize)]
pub(super) struct PathParams {
    id: Option<String>,
}

pub(super) async fn delete_todo(
    State(state): State<TodoState>,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let input_schema = InputSchema { id: path.id };

    println!("DELETE TODO -> input {input_schema:?}");

    let controller = DeleteController::new(input_schema, state.todo_store);

    if let Err(err) = controller.run().await.value() {
        let (status_code, message) = get_error_response_config(err);
        (status_code, Json(message)).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
}

fn get_error_response_config(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Validation(e) => {
            let validation_error = ValidationError::new(get_parse_error_field(&e), e.description());
            let error = ApiError::new("DTD-001", "Invalid input").with_details(validation_error);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error: ApiError<_> = ApiError::new("DTD-002", "Todo not found");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
        RunError::Internal => {
            let error: ApiError<ValidationError> =
                ApiError::new("DTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}

fn get_parse_error_field(error: &ParseError) -> &'static str {
    match error {
        ParseError::Id => "id",
    }
}
