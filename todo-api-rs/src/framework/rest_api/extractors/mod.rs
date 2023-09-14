mod string;
pub(super) use string::*;

use super::errors::{ApiError, ValidationError};

fn create_api_error(error: ValidationError) -> ApiError<ValidationError> {
    ApiError::new("VAL-001", "Invalid data format").with_details(error)
}
