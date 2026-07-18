//! Defines normalized television air dates and parsing.

use crate::utils::{DateError, validate_day, validate_month, validate_year};

/// AirDate captures date details. It is useful for capturing the premiere date of a movie or
/// television episode. It represents these details in a YYYY-MM-DD string format.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AirDate {
    year: u16,
    month: u8,
    day: u8,
}

impl AirDate {
    /// Constructs a new [AirDate] from the given year, month, and day.
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, DateError> {
        validate_year(year)?;
        validate_month(month)?;
        validate_day(year, month, day)?;
        Ok(Self { year, month, day })
    }

    /// Parses a date written as `YYYY-MM-DD`, `YYYY.MM.DD`, `YYYY/MM/DD`, or `YYYY MM DD` into an
    /// [AirDate].
    pub fn parse(s: &str) -> Result<Self, DateError> {
        let parts: Vec<&str> = s.split(['-', '.', '/', ' ']).collect();
        if parts.len() != 3 {
            return Err(DateError::InvalidInput(s.to_string()));
        }
        let (year, month, day) = (
            parts[0]
                .parse()
                .map_err(|_| DateError::InvalidInput(s.to_string()))?,
            parts[1]
                .parse()
                .map_err(|_| DateError::InvalidInput(s.to_string()))?,
            parts[2]
                .parse()
                .map_err(|_| DateError::InvalidInput(s.to_string()))?,
        );
        Self::new(year, month, day)
    }

    /// Returns the year of the [AirDate].
    pub const fn year(&self) -> u16 {
        self.year
    }

    /// Returns the month of the [AirDate].
    pub const fn month(&self) -> u8 {
        self.month
    }

    /// Returns the day of the [AirDate].
    pub const fn day(&self) -> u8 {
        self.day
    }
}

impl std::fmt::Display for AirDate {
    /// Converts the [AirDate] into a YYYY-MM-DD string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

crate::unit_tests!("air_date.test.rs");
