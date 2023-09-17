use uuid::{Error, Uuid};

#[derive(Clone, Debug, PartialEq)]
pub struct Id {
    uuid: Uuid,
}

impl Id {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Self { uuid }
    }

    pub fn parse_str(input: &str) -> Result<Self, Error> {
        Uuid::parse_str(input).map(Self::from)
    }

    pub fn as_string(&self) -> String {
        self.uuid.to_string()
    }

    pub fn uuid(&self) -> Uuid {
        // only works because Uuid struct derives Copy
        self.uuid
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self { uuid: value }
    }
}

impl serde::Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_string())
    }
}
