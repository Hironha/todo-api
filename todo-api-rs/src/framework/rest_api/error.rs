use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(super) struct ApiError<D: Serialize> {
    pub code: String,
    pub message: String,
    pub details: Option<D>,
}

impl<D: Serialize> ApiError<D> {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(self, details: D) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(super) struct ValidationError {
    field: String,
    description: String,
}

impl ValidationError {
    pub(super) fn new(field: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            description: description.into(),
        }
    }
}
