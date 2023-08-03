use serde::Deserialize;

use crate::application::functions::todo::CreatePayload;

#[derive(Debug, Deserialize)]
pub struct CreateTodoInput {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

impl CreateTodoInput {
    pub fn into_payload(self) -> Result<CreatePayload, String> {
        let title = match self.title {
            Some(t) => t,
            None => return Err("title is required".to_string()),
        };

        Ok(CreatePayload {
            title,
            description: self.description,
            todo_at: self.todo_at,
        })
    }
}
