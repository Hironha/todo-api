use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ValidationError {
    field: String,
    description: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiError<D: Serialize> {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<D>,
}

impl<D: Serialize> ApiError<D> {
    pub fn internal() -> Self {
        Self {
            code: String::from("Internal"),
            message: String::from("Internal server error"),
            details: None,
        }
    }

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
