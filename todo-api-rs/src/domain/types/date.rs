use thiserror::Error;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{Date as TimeDate, OffsetDateTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(time::Date);

impl Date {
    /// Create a new [`Date`] using current UTC date
    #[allow(dead_code)]
    pub fn now() -> Self {
        let dt = OffsetDateTime::now_utc();
        // probably safe to unwrap since it's using properties from OffsetDateTime
        Self(TimeDate::from_calendar_date(dt.year(), dt.month(), dt.day()).unwrap())
    }

    /// Get a equivalent struct from [`time`] crate
    pub fn time(&self) -> TimeDate {
        self.0
    }

    /// Stringify into `YYYY-MM-DD` using UTC date
    pub fn to_ymd(self) -> String {
        let ydm_description = format_description!("[year]-[month]-[day]");
        // probably safe to unwrap since it uses a well known/supported format
        self.0.format(ydm_description).unwrap()
    }

    pub fn parse_str(input: &str) -> Result<Self, ParseDateError> {
        let ymd_description = format_description!("[year]-[month]-[day]");
        time::Date::parse(input, ymd_description)
            .map(Self::from)
            .or(Err(ParseDateError::Invalid))
    }
}

impl From<time::Date> for Date {
    fn from(date: time::Date) -> Self {
        Self(date)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(OffsetDateTime);

impl DateTime {
    /// Create a new `DateTime` with the current date and time in UTC
    pub fn now() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    /// Get a equivalent struct from [`time`] crate
    pub fn time(&self) -> OffsetDateTime {
        self.0
    }

    /// Transform into a string following RFC 3339 pattern
    pub fn to_rfc3339(self) -> String {
        // probably safe to unwrap since it's using a well known/supported format
        self.0.format(&Rfc3339).unwrap()
    }
}

impl From<OffsetDateTime> for DateTime {
    fn from(date_time: OffsetDateTime) -> Self {
        Self(date_time)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseDateError {
    #[error("Date should be an UTC date on YYYY-MM-DD format")]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_to_ymd_works() {
        use std::str::FromStr;
        use time::Month;

        let now = OffsetDateTime::now_utc();
        let ymd = Date::now().to_ymd();
        assert!(Date::parse_str(&ymd).is_ok());

        let mut parts = ymd.split('-');
        assert_eq!(parts.next().map(i32::from_str), Some(Ok(now.year())));
        assert_eq!(
            Month::try_from(u8::from_str(parts.next().unwrap()).unwrap()),
            Ok(now.month())
        );
        assert_eq!(parts.next().map(u8::from_str), Some(Ok(now.day())));
    }

    #[test]
    fn parse_date_ymd_works() {
        let now = Date::now();
        let ymd = now.to_ymd();
        let parsed = Date::parse_str(&ymd);
        assert!(parsed.is_ok());
        assert_eq!(parsed, Ok(now));

        let ymd = String::from("2024-02-17");
        let parsed = Date::parse_str(&ymd);
        assert!(parsed.is_ok());
        assert_eq!(parsed.map(Date::to_ymd), Ok(ymd));

        let ymd = String::from("2024-2-17");
        let parsed = Date::parse_str(&ymd);
        assert!(parsed.is_err());
        assert_eq!(parsed, Err(ParseDateError::Invalid))
    }
}
