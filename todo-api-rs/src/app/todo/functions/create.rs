use super::TodoState;
use crate::core::todo::{create, Todo};

use axum::{extract::State, http::StatusCode, Json};

pub async fn create_todo(
    State(state): State<TodoState>,
    Json(payload): Json<create::CreatePayload>,
) -> (StatusCode, Json<Todo>) {
    let ctx = create::CreateContext {
        creator: state.todo_store,
    };
    let todo = create::create_todo(ctx, payload).await;
    (StatusCode::CREATED, Json(todo))
}
