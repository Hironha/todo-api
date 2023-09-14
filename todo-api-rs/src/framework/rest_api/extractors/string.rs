use serde_json::Value;

use super::create_api_error;
use crate::framework::rest_api::errors::{ApiError, ValidationError};

#[derive(Clone, Debug)]
pub struct OptionalString<'a>(&'a str);
impl<'a> OptionalString<'a> {
    pub fn extract(&self, value: &mut Value) -> Result<Option<String>, ApiError<ValidationError>> {
        let field = self.0;
        match value[field].take() {
            Value::String(v) => Ok(Some(v)),
            Value::Null => Ok(None),
            _ => {
                let description = format!("{} should be a string", field);
                return Err(create_api_error(ValidationError::new(field, description)));
            }
        }
    }
}

pub struct StringExtractor {}
impl StringExtractor {
    pub fn optional<'a>(field: &'a str) -> OptionalString<'a> {
        OptionalString(field)
    }
}
