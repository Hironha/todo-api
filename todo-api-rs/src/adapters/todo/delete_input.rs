use uuid::Uuid;

use crate::application::functions::todo::DeletePayload;

#[derive(Debug)]
pub struct DeleteTodoInput {
    pub id: Option<String>,
}

impl DeleteTodoInput {
    pub fn parse(self) -> Result<DeletePayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;
        if id.is_empty() {
            return Err("id should not be empty".to_string());
        }

        let uuid = Uuid::parse_str(&id).map_err(|_| "id should be a valid uuid".to_string())?;

        Ok(DeletePayload { id: uuid })
    }
}

mod test {
    #[test]
    fn parse_success() {
        let input = super::DeleteTodoInput {
            id: Some("id".to_string()),
        };

        assert!(input.parse().is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id = super::DeleteTodoInput { id: None };
        let none_id_payload = none_id.parse();

        assert!(none_id_payload.is_err());
        assert_eq!(none_id_payload.unwrap_err(), "id is required".to_string());

        let empty_id = super::DeleteTodoInput {
            id: Some("".to_string()),
        };
        let empty_id_payload = empty_id.parse();

        assert!(empty_id_payload.is_err());
        assert_eq!(
            empty_id_payload.unwrap_err(),
            "id should not be empty".to_string()
        );
    }
}
