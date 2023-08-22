use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::create::{CreateInput, ParseError};
use crate::application::functions::todo;
use crate::framework::rest_api::{ApiError, ValidationError};

impl From<ParseError> for ApiError<ValidationError> {
    fn from(error: ParseError) -> Self {
        let field = match error {
            ParseError::Title => "title",
            ParseError::TodoAt => "todoAt",
        };
        Self::from(ValidationError::new(field.into(), error.description()))
    }
}

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
        Err(err) => {
            let error = Json(ApiError::from(err));
            return (StatusCode::UNPROCESSABLE_ENTITY, error).into_response();
        }
    };

    let ctx = todo::CreateContext {
        store: state.todo_store,
    };
    let result = todo::create_todo(&ctx, payload).await;

    match result {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(message) => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
    }
}
