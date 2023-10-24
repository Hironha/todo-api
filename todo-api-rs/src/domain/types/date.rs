use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{error, OffsetDateTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(time::Date);

impl Date {
    pub fn into_date(self) -> time::Date {
        self.0
    }

    /// Stringify into YYYY-MM-DD using UTC date
    pub fn to_ymd(self) -> String {
        let ydm_description = format_description!("[year]-[month]-[day]");
        self.into_date().format(ydm_description).unwrap()
    }

    pub fn parse_str(input: &str) -> Result<Self, error::Parse> {
        let ymd_description = format_description!("[year]-[month]-[day]");
        time::Date::parse(input, ymd_description).map(Self::from)
    }
}

impl From<time::Date> for Date {
    /// Create a new `Date` instance from `time::Date`
    fn from(date: time::Date) -> Self {
        Self(date)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(OffsetDateTime);

impl DateTime {
    /// Create a new `DateTime`` with the current date and time in UTC.
    pub fn new() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    pub fn into_date_time(self) -> OffsetDateTime {
        self.0
    }

    /// Transform into a string following RFC 3339 pattern
    pub fn to_rfc3339(&self) -> String {
        self.0.format(&Rfc3339).unwrap()
    }
}

impl From<OffsetDateTime> for DateTime {
    /// Create a new `DateTime` instance from `OffsetDateTime`
    fn from(date_time: OffsetDateTime) -> Self {
        Self(date_time)
    }
}
