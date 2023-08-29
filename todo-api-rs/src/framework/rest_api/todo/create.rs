use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::create::{CreateInput, ParseError};
use crate::application::functions::todo;
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
    let input = CreateInput {
        title: body.title,
        description: body.description,
        todo_at: body.todo_at,
    };

    println!("CREATE TODO -> input: {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(error) => {
            let message = error.validation_error();
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(message)).into_response();
        }
    };

    let ctx = todo::CreateContext {
        store: state.todo_store,
    };

    match todo::create_todo(&ctx, payload).await {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(error) => {
            let (status_code, message) = error.api_error();
            (status_code, Json(message)).into_response()
        }
    }
}

impl ParseError {
    fn validation_error(&self) -> ApiError<ValidationError> {
        let field = match self {
            Self::Title => "title",
            Self::TodoAt => "todoAt",
        };
        ApiError::from(ValidationError::new(field.into(), self.description()))
    }
}

impl todo::CreateError {
    fn api_error(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "CTD-001".to_string(),
                    message: "Internal server error".to_string(),
                },
            ),
        }
    }
}
