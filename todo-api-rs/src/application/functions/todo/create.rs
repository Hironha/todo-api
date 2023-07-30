use crate::domain::todo::Todo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePayload {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

pub trait TodoCreator {
    fn create(&mut self, payload: CreatePayload) -> Result<Todo, String>;
}

pub struct CreateContext<T: TodoCreator> {
    pub creator: T,
}

pub async fn create_todo<T: TodoCreator>(
    mut ctx: CreateContext<T>,
    payload: CreatePayload,
) -> Result<Todo, String> {
    ctx.creator.create(payload)
}
