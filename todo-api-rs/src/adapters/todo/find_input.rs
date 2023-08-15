use uuid::Uuid;

use crate::application::functions::todo::FindPayload;

#[derive(Debug)]
pub struct FindTodoInput {
    pub id: Option<String>,
}

impl FindTodoInput {
    pub fn into_payload(self) -> Result<FindPayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;
        if id.is_empty() {
            return Err("id should not be empty".to_string());
        }

        let uuid = Uuid::parse_str(&id).map_err(|_| "id should be a valid uuid".to_string())?;

        Ok(FindPayload { id: uuid })
    }
}
