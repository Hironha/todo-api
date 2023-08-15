use time::{macros::format_description, Date};
use uuid::Uuid;

use crate::application::functions::todo::UpdatePayload;

#[derive(Debug)]
pub struct UpdateTodoInput {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl UpdateTodoInput {
    pub fn into_payload(self) -> Result<UpdatePayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;
        if id.is_empty() {
            return Err("id should not be empty".to_string());
        }

        let uuid = Uuid::parse_str(&id).map_err(|_| "id should be a valid uuid".to_string())?;

        let title = self
            .title
            .map(|t| if t.is_empty() { None } else { Some(t) })
            .ok_or("title should not be empty".to_string())?;

        let description = self.description.filter(|d| !d.is_empty());

        let todo_at = self
            .todo_at
            .map(|at| Date::parse(&at, &format_description!("[year]-[month]-[day]")))
            .transpose()
            .map_err(|_| "todo_at should be a date in the format YYYY-MM-DD".to_string())?;

        Ok(UpdatePayload {
            id: uuid,
            title,
            description,
            todo_at,
        })
    }
}
