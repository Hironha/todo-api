use uuid::Uuid;

use crate::application::functions::todo::FindPayload;

#[derive(Debug)]
pub struct FindTodoInput {
    pub id: Option<String>,
}

impl FindTodoInput {
    pub fn parse(self) -> Result<FindPayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;

        let uuid = Uuid::parse_str(&id).map_err(|_| "id should be a valid uuid".to_string())?;

        Ok(FindPayload { id: uuid })
    }
}

mod tests {
    #[test]
    fn parse_success() {
        let input = super::FindTodoInput {
            id: Some(uuid::Uuid::new_v4().to_string()),
        };

        assert!(input.parse().is_ok());
    }

    #[test]
    fn parse_fail() {
        let none_id = super::FindTodoInput { id: None };
        let none_id_payload = none_id.parse();

        assert!(none_id_payload.is_err_and(|e| e == "id is required"));

        let invalid_id = super::FindTodoInput {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_payload = invalid_id.parse();

        assert!(invalid_id_payload.is_err_and(|e| e == "id should be a valid uuid"));
    }
}
