//! Verifies air-date metadata behavior.

use super::*;

#[test]
fn new_ok() {
    let air_date = AirDate::new(1987, 5, 23).unwrap();
    assert_eq!(
        air_date,
        AirDate {
            year: 1987,
            month: 5,
            day: 23
        }
    );
}

#[test]
fn new_err() {
    assert_eq!(AirDate::new(0, 5, 23), Err(DateError::OutOfRangeYear(0)));
    assert_eq!(
        AirDate::new(1987, 0, 23),
        Err(DateError::OutOfRangeMonth(0))
    );
    assert_eq!(AirDate::new(1987, 5, 0), Err(DateError::OutOfRangeDay(0)));
}

#[test]
fn parse() {
    let air_date = AirDate::parse("1987-05-23").unwrap();
    assert_eq!(
        air_date,
        AirDate {
            year: 1987,
            month: 5,
            day: 23
        }
    );
}

#[test]
fn parse_common_filename_separators() {
    for value in ["1987.05.23", "1987/05/23", "1987 05 23"] {
        assert_eq!(
            AirDate::parse(value).unwrap(),
            AirDate::new(1987, 5, 23).unwrap()
        );
    }
}

#[test]
fn year() {
    let air_date = AirDate {
        year: 1987,
        month: 5,
        day: 23,
    };
    assert_eq!(air_date.year(), 1987);
}

#[test]
fn month() {
    let air_date = AirDate {
        year: 1987,
        month: 5,
        day: 23,
    };
    assert_eq!(air_date.month(), 5);
}

#[test]
fn day() {
    let air_date = AirDate {
        year: 1987,
        month: 5,
        day: 23,
    };
    assert_eq!(air_date.day(), 23);
}

#[test]
fn to_string() {
    let air_date = AirDate {
        year: 1987,
        month: 5,
        day: 23,
    };
    assert_eq!(air_date.to_string(), "1987-05-23");
}
