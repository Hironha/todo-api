use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::create::{CreateController, RunError};
use crate::adapters::dtos::todo::create::{InputSchema, ParseError};
use crate::framework::rest_api::{ApiError, ValidationError};

#[derive(Deserialize)]
pub(super) struct RequestBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

pub(super) async fn create_todo(
    State(state): State<TodoState>,
    Json(body): Json<RequestBody>,
) -> impl IntoResponse {
    let input_schema = InputSchema {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
    };

    println!("CREATE TODO -> input: {input_schema:?}");

    let controller = CreateController::new(input_schema, state.todo_store);
    let output = match controller.run().await {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = get_error_response_config(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::CREATED, Json(output)).into_response()
}

fn get_error_response_config(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Validation(e) => {
            let validation_error = ValidationError::new(get_parse_error_field(&e), e.description());
            let error = ApiError::new("CTD-001", "Invalid input").with_details(validation_error);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::Internal => {
            let error = ApiError::new("CTD-002", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}

fn get_parse_error_field(error: &ParseError) -> &'static str {
    match error {
        ParseError::Title => "title",
        ParseError::TodoAt => "todoAt",
    }
}
