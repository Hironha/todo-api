use crate::application::functions::todo::FindPayload;
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
pub struct FindInput {
    pub id: Option<String>,
}

impl FindInput {
    pub fn parse(self) -> Result<FindPayload, ParseError> {
        let id = self
            .id
            .map(|id| Id::parse_str(&id))
            .ok_or(ParseError::Id)?
            .map_err(|_| ParseError::Id)?;

        Ok(FindPayload { id })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_success() {
        let input = super::FindInput {
            id: Some(super::Id::new().as_string()),
        };

        assert!(input.parse().is_ok());
    }

    #[test]
    fn parse_fail() {
        let none_id = super::FindInput { id: None };
        let none_id_payload = none_id.parse();

        assert!(none_id_payload.is_err_and(|e| e == super::ParseError::Id));

        let invalid_id = super::FindInput {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_payload = invalid_id.parse();

        assert!(invalid_id_payload.is_err_and(|e| e == super::ParseError::Id));
    }
}
