use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ValidationError {
    field: std::sync::Arc<str>,
    description: std::sync::Arc<str>,
}

impl ValidationError {
    pub fn new(
        field: impl Into<std::sync::Arc<str>>,
        description: impl Into<std::sync::Arc<str>>,
    ) -> Self {
        Self {
            field: field.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiError<D: Serialize> {
    pub code: std::sync::Arc<str>,
    pub message: std::sync::Arc<str>,
    pub details: Option<D>,
}

impl<D: Serialize> ApiError<D> {
    pub fn new(
        code: impl Into<std::sync::Arc<str>>,
        message: impl Into<std::sync::Arc<str>>,
    ) -> Self {
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
