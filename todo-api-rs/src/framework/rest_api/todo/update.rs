use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::{
    adapters::todo::update::{ParseError, UpdateInput},
    application::functions::todo,
};

impl ParseError {
    fn message(&self) -> String {
        match self {
            Self::Id => format!("id: {}", self.description()),
            Self::Title => format!("title: {}", self.description()),
            Self::TodoAt => format!("todoAt: {}", self.description()),
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
        Err(err) => return (StatusCode::UNPROCESSABLE_ENTITY, err.message()).into_response(),
    };

    let ctx = todo::UpdateContext {
        store: state.todo_store,
    };
    let result = todo::update_todo(&ctx, payload).await;

    let todo = match result {
        Ok(todo) => todo,
        Err(message) => return (StatusCode::NOT_FOUND, message).into_response(),
    };

    println!("UPDATE TODO -> updated: {todo:?}");

    (StatusCode::OK, Json(todo)).into_response()
}
