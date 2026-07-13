//! Organizes internal metadata parsing utilities.

mod date;

pub use date::{DateError, validate_day, validate_month, validate_year};
