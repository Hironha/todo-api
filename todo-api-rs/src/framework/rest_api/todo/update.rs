use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::TodoState;
use crate::{adapters::todo::update_input::UpdateTodoInput, application::functions::todo};

#[derive(Deserialize)]
pub struct UpdateTodoPath {
  id: Option<String>
}

#[derive(Deserialize)]
pub struct UpdateTodoBody {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize="todoAt"))]
    todo_at: Option<String>,
}

pub async fn update_todo(
    State(state): State<TodoState>,
    Path(path): Path<UpdateTodoPath>,
    Json(body): Json<UpdateTodoBody>,
) -> impl IntoResponse {
    let input = UpdateTodoInput {
        id: path.id,
        description: body.description,
        title: body.title,
        todo_at: body.todo_at,
    };

    println!("UPDATE TODO -> input: {input:?}");

    let payload = match input.into_payload() {
        Ok(payload) => payload,
        Err(message) => return (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
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
