use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

use super::TodoState;
use crate::adapters::controllers::todo::create::CreateController;
use crate::adapters::dtos::todo::create::{InputSchema, ParseError, RunError};
use crate::framework::rest_api::errors::{ApiError, ValidationError};
use crate::framework::rest_api::extractors::StringExtractor;

pub(super) async fn create_todo(
    State(state): State<TodoState>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    println!("CREATE TODO -> body: {body:#?}");

    let input_schema = match extract_input_schema(body) {
        Ok(input) => input,
        Err(err) => return (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response(),
    };
    let controller = CreateController::new(input_schema, state.todo_store);

    let output = match controller.run().await.value() {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::CREATED, Json(output)).into_response()
}

fn extract_input_schema(mut body: Value) -> Result<InputSchema, ApiError<ValidationError>> {
    let title = StringExtractor::optional("title").extract(&mut body)?;
    let description = StringExtractor::optional("description").extract(&mut body)?;
    let todo_at = StringExtractor::optional("todoAt").extract(&mut body)?;

    Ok(InputSchema {
        title,
        description,
        todo_at,
    })
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
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
