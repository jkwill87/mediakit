//! Parses supported release-date representations.

use thiserror::Error;

const MIN_YEAR: u16 = 1900;
const MAX_YEAR: u16 = 2099;

const MIN_MONTH: u8 = 1;
const MAX_MONTH: u8 = 12;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DateError {
    #[error("Invalid year: {0}")]
    OutOfRangeYear(u16),

    #[error("Invalid month: {0}")]
    OutOfRangeMonth(u8),

    #[error("Invalid day: {0}")]
    OutOfRangeDay(u8),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub fn validate_year(year: u16) -> Result<u16, DateError> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        Err(DateError::OutOfRangeYear(year))
    } else {
        Ok(year)
    }
}

pub fn validate_month(month: u8) -> Result<u8, DateError> {
    if !(MIN_MONTH..=MAX_MONTH).contains(&month) {
        Err(DateError::OutOfRangeMonth(month))
    } else {
        Ok(month)
    }
}

pub const fn validate_day(year: u16, month: u8, day: u8) -> Result<u8, DateError> {
    let max_day = match month {
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 if year.is_multiple_of(4) => 29,
        2 => 28,
        _ => return Err(DateError::OutOfRangeDay(day)),
    };
    match (day < 1) || (day > max_day) {
        true => Err(DateError::OutOfRangeDay(day)),
        false => Ok(day),
    }
}

crate::unit_tests!("date.test.rs");
