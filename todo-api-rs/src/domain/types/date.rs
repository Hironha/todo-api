use time::{error, format_description::well_known::Rfc3339, macros::format_description};

#[derive(Clone, Debug)]
pub struct Date {
    date: time::Date,
}

impl Date {
    pub fn date(&self) -> time::Date {
        self.date
    }

    /// stringifies into Y-M-D
    pub fn ymd(&self) -> String {
        let ydm_description = format_description!("[year]-[month]-[day]");
        self.date().format(ydm_description).unwrap()
    }

    pub fn parse_str(input: &str) -> Result<Self, error::Parse> {
        let ymd_description = format_description!("[year]-[month]-[day]");
        time::Date::parse(input, ymd_description).map(Self::from)
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}

impl AsRef<time::Date> for Date {
    fn as_ref(&self) -> &time::Date {
        &self.date
    }
}

impl From<time::Date> for Date {
    fn from(date: time::Date) -> Self {
        Self{ date }
    }
}

impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.ymd())
    }
}

#[derive(Clone, Debug)]
pub struct DateTime {
    time: time::OffsetDateTime,
}

impl DateTime {
    /// Transforms into a string following RFC 3339 pattern
    pub fn rfc3339(&self) -> String {
        self.time.format(&Rfc3339).unwrap()
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl From<time::OffsetDateTime> for DateTime {
    fn from(date_time: time::OffsetDateTime) -> Self {
        Self { time: date_time }
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.rfc3339())
    }
}
