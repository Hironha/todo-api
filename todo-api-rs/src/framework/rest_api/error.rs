use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(super) struct ApiError<M: Serialize> {
    pub code: String,
    pub message: M,
}

impl From<ValidationError> for ApiError<ValidationError> {
    fn from(error: ValidationError) -> Self {
        Self {
            code: "VAL-001".to_string(),
            message: error,
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
