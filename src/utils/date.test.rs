//! Verifies release-date parsing.

use super::*;

#[test]
fn test_validate_year_in_range() {
    assert_eq!(validate_year(MIN_YEAR), Ok(MIN_YEAR));
    assert_eq!(validate_year(MAX_YEAR), Ok(MAX_YEAR));
}

#[test]
fn test_validate_year_out_of_range() {
    const TOO_LOW: u16 = MIN_YEAR - 1;
    const TOO_HIGH: u16 = MAX_YEAR + 1;
    assert_eq!(
        validate_year(TOO_LOW),
        Err(DateError::OutOfRangeYear(TOO_LOW))
    );
    assert_eq!(
        validate_year(TOO_HIGH),
        Err(DateError::OutOfRangeYear(TOO_HIGH))
    );
}

#[test]
fn test_validate_month_in_range() {
    assert_eq!(validate_month(MIN_MONTH), Ok(MIN_MONTH));
    assert_eq!(validate_month(MAX_MONTH), Ok(MAX_MONTH));
}

#[test]
fn test_validate_month_out_of_range() {
    const TOO_LOW: u8 = MIN_MONTH - 1;
    const TOO_HIGH: u8 = MAX_MONTH + 1;
    assert_eq!(
        validate_month(TOO_LOW),
        Err(DateError::OutOfRangeMonth(TOO_LOW))
    );
    assert_eq!(
        validate_month(TOO_HIGH),
        Err(DateError::OutOfRangeMonth(TOO_HIGH))
    );
}

#[test]
fn test_validate_day_in_range() {
    assert_eq!(validate_day(2000, 1, 1), Ok(1));
    assert_eq!(validate_day(2000, 1, 31), Ok(31));
    assert_eq!(validate_day(2000, 2, 29), Ok(29));
    assert_eq!(validate_day(2000, 2, 28), Ok(28));
    assert_eq!(validate_day(2000, 3, 31), Ok(31));
    assert_eq!(validate_day(2000, 4, 30), Ok(30));
    assert_eq!(validate_day(2000, 5, 31), Ok(31));
    assert_eq!(validate_day(2000, 6, 30), Ok(30));
    assert_eq!(validate_day(2000, 7, 31), Ok(31));
    assert_eq!(validate_day(2000, 8, 31), Ok(31));
    assert_eq!(validate_day(2000, 9, 30), Ok(30));
    assert_eq!(validate_day(2000, 10, 31), Ok(31));
    assert_eq!(validate_day(2000, 11, 30), Ok(30));
    assert_eq!(validate_day(2000, 12, 31), Ok(31));
}

#[test]
fn test_validate_day_out_of_range() {
    assert_eq!(validate_day(2000, 1, 0), Err(DateError::OutOfRangeDay(0)));
    assert_eq!(validate_day(2000, 1, 32), Err(DateError::OutOfRangeDay(32)));
    assert_eq!(validate_day(2000, 2, 30), Err(DateError::OutOfRangeDay(30)));
    assert_eq!(validate_day(2001, 2, 29), Err(DateError::OutOfRangeDay(29)));
    assert_eq!(validate_day(2000, 3, 32), Err(DateError::OutOfRangeDay(32)));
    assert_eq!(validate_day(2000, 4, 31), Err(DateError::OutOfRangeDay(31)));
    assert_eq!(validate_day(2000, 5, 32), Err(DateError::OutOfRangeDay(32)));
    assert_eq!(validate_day(2000, 6, 31), Err(DateError::OutOfRangeDay(31)));
    assert_eq!(validate_day(2000, 7, 32), Err(DateError::OutOfRangeDay(32)));
    assert_eq!(validate_day(2000, 8, 32), Err(DateError::OutOfRangeDay(32)));
    assert_eq!(validate_day(2000, 9, 31), Err(DateError::OutOfRangeDay(31)));
    assert_eq!(
        validate_day(2000, 10, 32),
        Err(DateError::OutOfRangeDay(32))
    );
    assert_eq!(
        validate_day(2000, 11, 31),
        Err(DateError::OutOfRangeDay(31))
    );
    assert_eq!(
        validate_day(2000, 12, 32),
        Err(DateError::OutOfRangeDay(32))
    );
}
