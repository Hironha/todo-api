use serde::Deserialize;

use crate::application::functions::todo::GetTodoPayload;

#[derive(Debug, Deserialize)]
pub struct GetTodoInput {
    id: Option<String>,
}

impl GetTodoInput {
    pub fn into_payload(self) -> Result<GetTodoPayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;
        if id.is_empty() {
            return Err("id should not be empty".to_string());
        }

        Ok(GetTodoPayload { id })
    }
}
