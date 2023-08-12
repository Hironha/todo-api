use time::{
    format_description::well_known::Rfc3339, macros::format_description, Date, OffsetDateTime,
};

#[derive(Clone, Debug)]
pub struct SerializableDate(Date);

impl SerializableDate {
    pub fn to_date(&self) -> Date {
        self.0
    }

    pub fn to_ymd(&self) -> String {
        let ymd_format = format_description!("[year]-[month]-[day]");
        self.to_date().format(ymd_format).unwrap()
    }
}

impl AsRef<Date> for SerializableDate {
    fn as_ref(&self) -> &Date {
        &self.0
    }
}

impl From<Date> for SerializableDate {
    fn from(date: Date) -> Self {
        Self(date)
    }
}

impl serde::Serialize for SerializableDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_ymd())
    }
}

#[derive(Clone, Debug)]
pub struct SerializableDateTime(OffsetDateTime);

impl SerializableDateTime {
    pub const fn to_date_time(&self) -> OffsetDateTime {
        self.0
    }
}

impl From<OffsetDateTime> for SerializableDateTime {
    fn from(date_time: OffsetDateTime) -> Self {
        Self(date_time)
    }
}

impl serde::Serialize for SerializableDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let date_time = self.to_date_time();
        let iso_date_time = date_time.format(&Rfc3339).unwrap();
        serializer.serialize_str(&iso_date_time)
    }
}
