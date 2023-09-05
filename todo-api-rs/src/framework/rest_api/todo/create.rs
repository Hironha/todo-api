use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::create::{CreateController, RunError};
use crate::adapters::dtos::todo::create::{Input, InputSchema, ParseError};
use crate::framework::rest_api::{ApiError, ValidationError};

#[derive(Deserialize)]
pub(super) struct CreateTodoBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

pub(super) async fn create_todo(
    State(state): State<TodoState>,
    Json(body): Json<CreateTodoBody>,
) -> impl IntoResponse {
    let input_schema = InputSchema {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
    };

    println!("CREATE TODO -> input: {input_schema:?}");

    let input = match Input::parse(input_schema) {
        Ok(input) => input,
        Err(err) => {
            let error = err.api_error();
            return (StatusCode::BAD_REQUEST, Json(error)).into_response();
        }
    };

    let controller = CreateController::new(state.todo_store);

    match controller.run(input).await {
        Ok(output) => (StatusCode::CREATED, Json(output)).into_response(),
        Err(err) => {
            let (status, message) = err.response_parts();
            (status, Json(message)).into_response()
        }
    }
}

impl ParseError {
    fn api_error(&self) -> ApiError<ValidationError> {
        let field = match self {
            Self::Title => "title",
            Self::TodoAt => "todoAt",
        };
        ApiError::from(ValidationError::new(field.into(), self.description()))
    }
}

impl RunError {
    fn response_parts(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "CTD-001".into(),
                    message: "Internal server error".into(),
                },
            ),
        }
    }
}
