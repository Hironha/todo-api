use crate::application::functions::todo::DeletePayload;
use crate::domain::types::Id;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Id,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::Id => "required and it should be a valid uuid".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct DeleteInput {
    pub id: Option<String>,
}

impl DeleteInput {
    pub fn parse(self) -> Result<DeletePayload, ParseError> {
        let id = self
            .id
            .map(|id| Id::parse_str(&id))
            .ok_or(ParseError::Id)?
            .map_err(|_| ParseError::Id)?;

        Ok(DeletePayload { id })
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
        assert_eq!(none_id_payload.unwrap_err(), super::ParseError::Id);

        let invalid_id = super::DeleteInput {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_payload = invalid_id.parse();

        assert!(invalid_id_payload.is_err());
        assert_eq!(invalid_id_payload.unwrap_err(), super::ParseError::Id);
    }
}
