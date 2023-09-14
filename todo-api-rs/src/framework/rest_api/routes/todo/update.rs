use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TodoState;
use crate::adapters::controllers::todo::update::UpdateController;
use crate::adapters::dtos::todo::update::{InputSchema, ParseError, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};
use crate::framework::rest_api::extractors::StringExtractor;

pub(super) async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<Value>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    println!("UPDATE TODO -> path: {path:#?}");
    println!("UPDATE TODO -> body: {body:#?}");

    let input_schema = match extract_input_schema(path, body) {
        Ok(input) => input,
        Err(err) => return (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response(),
    };
    let controller = UpdateController::new(input_schema, state.todo_store);

    let output = match controller.run().await.value() {
        Ok(output) => output,
        Err(err) => {
            let (status_code, message) = config_error_response(err);
            return (status_code, Json(message)).into_response();
        }
    };

    println!("UPDATE TODO -> updated: {output:?}");

    (StatusCode::OK, Json(output)).into_response()
}

fn extract_input_schema(
    mut path: Value,
    mut body: Value,
) -> Result<InputSchema, ApiError<ValidationError>> {
    let id = StringExtractor::optional("id").extract(&mut path)?;
    let title = StringExtractor::optional("title").extract(&mut body)?;
    let description = StringExtractor::optional("description").extract(&mut body)?;
    let todo_at = StringExtractor::optional("todoAt").extract(&mut body)?;

    Ok(InputSchema {
        id,
        title,
        description,
        todo_at,
    })
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
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
