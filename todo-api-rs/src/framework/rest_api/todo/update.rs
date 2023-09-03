use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::update::{UpdateController, RunError};
use crate::adapters::dtos::todo::update::{Input, InputSchema, ParseError};
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
    let input_schema = InputSchema {
        id: path.id,
        description: body.description,
        title: body.title,
        todo_at: body.todo_at,
    };

    println!("UPDATE TODO -> input: {input_schema:?}");

    let input = match Input::parse(input_schema) {
        Ok(input) => input,
        Err(error) => {
            let message = error.validation_error();
            return (StatusCode::BAD_REQUEST, Json(message)).into_response();
        }
    };

    let controller = UpdateController::new(state.todo_store);
    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(error) => {
            let (status_code, message) = error.api_error();
            return (status_code, Json(message)).into_response();
        }
    };

    println!("UPDATE TODO -> updated: {output:?}");

    (StatusCode::OK, Json(output)).into_response()
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

impl RunError {
    fn api_error(&self) -> (StatusCode, ApiError<String>) {
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ApiError {
                    code: "UTD-001".to_string(),
                    message: "Todo not found".to_string(),
                },
            ),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    code: "UTD-002".to_string(),
                    message: "Internal server error".to_string(),
                },
            ),
        }
    }
}
