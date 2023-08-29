use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::update::{ParseError, UpdateInput};
use crate::application::functions::todo;
use crate::framework::rest_api::{ApiError, ValidationError};

#[derive(Deserialize)]
pub(super) struct UpdateTodoPath {
    id: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct UpdateTodoBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

pub(super) async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<UpdateTodoPath>,
    Json(body): Json<UpdateTodoBody>,
) -> impl IntoResponse {
    let input = UpdateInput {
        id: path.id,
        description: body.description,
        title: body.title,
        todo_at: body.todo_at,
    };

    println!("UPDATE TODO -> input: {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(error) => {
            let message = error.validation_error();
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(message)).into_response();
        }
    };

    let ctx = todo::UpdateContext {
        store: state.todo_store,
    };

    let todo = match todo::update_todo(&ctx, payload).await {
        Ok(todo) => todo,
        Err(error) => {
            let (status_code, message) = error.api_error();
            return (status_code, Json(message)).into_response();
        }
    };

    println!("UPDATE TODO -> updated: {todo:?}");

    (StatusCode::OK, Json(todo)).into_response()
}

impl ParseError {
    fn validation_error(&self) -> ApiError<ValidationError> {
        let field = match self {
            Self::Id => "id",
            Self::Title => "title",
            Self::TodoAt => "todoAt",
        };
        ApiError::from(ValidationError::new(field.into(), self.description()))
    }
}

impl todo::UpdateError {
    fn api_error(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ApiError {
                    code: "UTD-001".to_string(),
                    message: "Todo not found".to_string(),
                },
            ),
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "UTD-002".to_string(),
                    message: "Internal server error".to_string(),
                },
            ),
        }
    }
}
