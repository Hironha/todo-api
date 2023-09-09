use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::update::{RunError, UpdateController};
use crate::adapters::dtos::todo::update::{InputSchema, ParseError};
use crate::framework::rest_api::{ApiError, ValidationError};

#[derive(Deserialize)]
pub(super) struct PathParams {
    id: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct RequestBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

pub(super) async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<PathParams>,
    Json(body): Json<RequestBody>,
) -> impl IntoResponse {
    let input_schema = InputSchema {
        id: path.id,
        description: body.description,
        title: body.title,
        todo_at: body.todo_at,
    };

    println!("UPDATE TODO -> input: {input_schema:?}");

    let controller = UpdateController::new(input_schema, state.todo_store);
    let output = match controller.run().await {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = get_error_response_config(err);
            return (status_code, Json(message)).into_response();
        }
    };

    println!("UPDATE TODO -> updated: {output:?}");

    (StatusCode::OK, Json(output)).into_response()
}

fn get_error_response_config(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Validation(e) => {
            let validation_error = ValidationError::new(get_parse_error_field(&e), e.description());
            let error = ApiError::new("UTD-001", "Invalid input").with_details(validation_error);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::NotFound => {
            let error = ApiError::new("UTD-002", "Todo not found");
            (StatusCode::NOT_FOUND, error)
        }
        RunError::Internal => {
            let error = ApiError::new("UTD-003", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}

fn get_parse_error_field(error: &ParseError) -> &'static str {
    match error {
        ParseError::Id => "id",
        ParseError::Title => "title",
        ParseError::TodoAt => "todoAt",
    }
}
