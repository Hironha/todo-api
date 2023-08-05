use serde::Deserialize;

use crate::application::functions::todo::UpdatePayload;

#[derive(Debug, Deserialize)]
pub struct UpdateTodoInput {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    pub todo_at: Option<String>,
}

impl UpdateTodoInput {
    pub fn into_payload(self) -> Result<UpdatePayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;
        if id.is_empty() {
            return Err("id should not be empty".to_string());
        }

        let title = self
            .title
            .map(|t| if t.is_empty() { None } else { Some(t) })
            .ok_or("title should not be empty".to_string())?;

        let description = self.description.filter(|d| !d.is_empty());

        Ok(UpdatePayload {
            id,
            title,
            description,
            todo_at: self.todo_at,
        })
    }
}
