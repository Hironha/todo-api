use std::fmt::Display;
use uuid::{Error, Uuid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<Self, Error> {
        Uuid::parse_str(input).map(Self::from)
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<Uuid> for Id {
    /// Creates a new `Id` instance from `uuid::Uuid`
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl serde::Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
