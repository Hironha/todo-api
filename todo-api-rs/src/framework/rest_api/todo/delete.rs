use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::delete::{DeleteInput, ParseError};
use crate::application::functions::todo;
use crate::framework::rest_api::{ApiError, ValidationError};

impl From<ParseError> for ApiError<ValidationError> {
    fn from(error: ParseError) -> Self {
        let field = match error {
            ParseError::Id => "id",
        };
        Self::from(ValidationError::new(field.into(), error.description()))
    }
}

impl todo::DeleteError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalError => StatusCode::NOT_FOUND,
            Self::NotFound => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<todo::DeleteError> for ApiError<String> {
    fn from(error: todo::DeleteError) -> Self {
        match error {
            todo::DeleteError::NotFound => ApiError {
                code: "DTD-001".to_string(),
                message: "Todo not found".to_string(),
            },
            todo::DeleteError::InternalError => ApiError {
                code: "DTD-002".to_string(),
                message: "Internal server error".to_string(),
            },
        }
    }
}

#[derive(Deserialize)]
pub(super) struct DeleteTodoPath {
    id: Option<String>,
}

pub(super) async fn delete_todo(
    State(state): State<TodoState>,
    Path(path): Path<DeleteTodoPath>,
) -> impl IntoResponse {
    let input = DeleteInput { id: path.id };

    println!("DELETE TODO -> input {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(err) => {
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(ApiError::from(err))).into_response();
        }
    };

    let ctx = todo::DeleteContext {
        store: state.todo_store,
    };

    if let Err(error) = todo::delete_todo(&ctx, &payload).await {
        (error.status_code(), Json(ApiError::from(error))).into_response()
    } else {
        (StatusCode::NO_CONTENT).into_response()
    }
}
