use crate::application::functions::todo::DeletePayload;
use crate::domain::types::Id;

#[derive(Debug)]
pub struct DeleteInput {
    pub id: Option<String>,
}

impl DeleteInput {
    pub fn parse(self) -> Result<DeletePayload, String> {
        let id = self.id.ok_or("id is required".to_string())?;

        let uuid = Id::parse_str(&id).map_err(|_| "id should be a valid uuid".to_string())?;

        Ok(DeletePayload { id: uuid })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_success() {
        let input = super::DeleteInput {
            id: Some(super::Id::new().as_string()),
        };

        assert!(input.parse().is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id = super::DeleteInput { id: None };
        let none_id_payload = none_id.parse();

        assert!(none_id_payload.is_err());
        assert_eq!(none_id_payload.unwrap_err(), "id is required".to_string());

        let invalid_id = super::DeleteInput {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_payload = invalid_id.parse();

        assert!(invalid_id_payload.is_err());
        assert_eq!(
            invalid_id_payload.unwrap_err(),
            "id should be a valid uuid".to_string()
        );
    }
}
