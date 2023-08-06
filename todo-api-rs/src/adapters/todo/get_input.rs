use crate::application::functions::todo::GetPayload;

#[derive(Debug)]
pub struct GetTodoInput {
    pub id: Option<String>,
}

impl GetTodoInput {
    pub fn into_payload(self) -> Result<GetPayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;
        if id.is_empty() {
            return Err("id should not be empty".to_string());
        }

        Ok(GetPayload { id })
    }
}
