use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::adapters::todo::find::{FindInput, ParseError};
use crate::application::functions::todo;
use crate::framework::rest_api::error::{ApiError, ValidationError};

impl From<ParseError> for ApiError<ValidationError> {
    fn from(error: ParseError) -> Self {
        let field = match error {
            ParseError::Id => "id",
        };
        Self::from(ValidationError::new(field.into(), error.description()))
    }
}

impl todo::FindError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<todo::FindError> for ApiError<String> {
    fn from(error: todo::FindError) -> Self {
        match error {
            todo::FindError::NotFound => Self {
                code: "FTD-001".to_string(),
                message: "Todo not found".to_string(),
            },
            todo::FindError::InternalError => Self {
                code: "FTD-002".to_string(),
                message: "Internal server error".to_string(),
            },
        }
    }
}

#[derive(Deserialize)]
pub(super) struct GetTodoPath {
    id: Option<String>,
}

pub(super) async fn find_todo(
    State(state): State<TodoState>,
    Path(path): Path<GetTodoPath>,
) -> impl IntoResponse {
    let input = FindInput { id: path.id };

    println!("GET TODO -> input: {input:?}");

    let payload = match input.parse() {
        Ok(payload) => payload,
        Err(err) => {
            let error = Json(ApiError::from(err));
            return (StatusCode::UNPROCESSABLE_ENTITY, error).into_response();
        }
    };

    let ctx = todo::GetContext {
        store: state.todo_store,
    };
    let result = todo::find_todo(ctx, &payload).await;

    let todo = match result {
        Ok(todo) => todo,
        Err(err) => return (err.status_code(), Json(ApiError::from(err))).into_response(),
    };

    (StatusCode::OK, Json(todo)).into_response()
}
