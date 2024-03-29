use std::fmt;

use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<Self, ()> {
        Uuid::parse_str(input).map(Self).or(Err(()))
    }

    /// Get equivalent [`uuid::Uuid`] struct of [`Id`]
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
