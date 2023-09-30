use uuid::{Error, Uuid};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Id(Uuid);

impl Id {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<Self, Error> {
        Uuid::parse_str(input).map(Self::from)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for Id {
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
