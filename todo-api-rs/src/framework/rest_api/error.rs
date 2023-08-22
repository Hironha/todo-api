use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(super) struct ApiError<M: Serialize> {
    code: String,
    message: M,
    short_message: String,
}

impl From<ValidationError> for ApiError<ValidationError> {
    fn from(error: ValidationError) -> Self {
        Self {
            code: "VAL-001".to_string(),
            message: error,
            short_message: "validationError".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(super) struct ValidationError {
    field: String,
    description: String,
}

impl ValidationError {
    pub(super) const fn new(field: String, description: String) -> Self {
        Self { field, description }
    }
}
