use time::{error, format_description::well_known::Rfc3339, macros::format_description};

#[derive(Clone, Debug)]
pub struct Date(time::Date);

impl Date {
    pub fn to_date(&self) -> time::Date {
        self.0
    }

    pub fn to_ymd(&self) -> String {
        let ydm_description = format_description!("[year]-[month]-[day]");
        self.to_date().format(ydm_description).unwrap()
    }

    pub fn parse(input: &str) -> Result<Self, error::Parse> {
        let ymd_description = format_description!("[year]-[month]-[day]");
        time::Date::parse(input, ymd_description).map(Self)
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl AsRef<time::Date> for Date {
    fn as_ref(&self) -> &time::Date {
        &self.0
    }
}

impl From<time::Date> for Date {
    fn from(date: time::Date) -> Self {
        Self(date)
    }
}

impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_ymd())
    }
}

#[derive(Clone, Debug)]
pub struct DateTime(time::OffsetDateTime);

impl DateTime {
    pub const fn to_date_time(&self) -> time::OffsetDateTime {
        self.0
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<time::OffsetDateTime> for DateTime {
    fn from(date_time: time::OffsetDateTime) -> Self {
        Self(date_time)
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let date_time = self.to_date_time();
        let iso_date_time = date_time.format(&Rfc3339).unwrap();
        serializer.serialize_str(&iso_date_time)
    }
}
