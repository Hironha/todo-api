use std::error;

use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct JsonError {
    status: u16,
    src: Option<Box<dyn error::Error>>,
    pub content: Value,
}

impl JsonError {
    pub fn new(status: u16, content: Content) -> Self {
        Self {
            status,
            src: None,
            content: json!(content),
        }
    }

    pub fn internal() -> Self {
        let content = Content::internal();
        Self {
            status: 500,
            src: None,
            content: json!(content),
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn src(&self) -> Option<&dyn error::Error> {
        self.src.as_deref()
    }

    pub fn with_src(mut self, src: impl Into<Box<dyn error::Error>>) -> Self {
        self.src = Some(src.into());
        self
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Content {
    code: String,
    message: String,
}

impl Content {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    fn internal() -> Self {
        Self {
            code: String::from("InternalError"),
            message: String::from("Internal server error"),
        }
    }
}
