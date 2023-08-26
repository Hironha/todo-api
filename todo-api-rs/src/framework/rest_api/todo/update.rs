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

impl From<ParseError> for ApiError<ValidationError> {
    fn from(error: ParseError) -> Self {
        let field = match error {
            ParseError::Id => "id",
            ParseError::Title => "title",
            ParseError::TodoAt => "todoAt",
        };
        Self::from(ValidationError::new(field.into(), error.description()))
    }
}

impl todo::UpdateError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<todo::UpdateError> for ApiError<String> {
    fn from(error: todo::UpdateError) -> Self {
        match error {
            todo::UpdateError::NotFound => Self {
                code: "UTD-001".to_string(),
                message: "Todo not found".to_string(),
            },
            todo::UpdateError::InternalError => Self {
                code: "UTD-002".to_string(),
                message: "Internal server error".to_string(),
            },
        }
    }
}

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
        Err(err) => {
            let error = Json(ApiError::from(err));
            return (StatusCode::UNPROCESSABLE_ENTITY, error).into_response();
        }
    };

    let ctx = todo::UpdateContext {
        store: state.todo_store,
    };
    let result = todo::update_todo(&ctx, payload).await;

    let todo = match result {
        Ok(todo) => todo,
        Err(err) => return (err.status_code(), Json(ApiError::from(err))).into_response(),
    };

    println!("UPDATE TODO -> updated: {todo:?}");

    (StatusCode::OK, Json(todo)).into_response()
}
