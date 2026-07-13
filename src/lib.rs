//! Parses media metadata from filenames and file properties.

#![deny(missing_docs)]

// Allows you to customize the logo
// #![doc(html_logo_url = "path_to_logo", html_favicon_url = "path_to_favicon")] // TODO uncomment

pub mod inspect;
mod macros;
pub(crate) use macros::unit_tests;
pub mod meta;
pub mod probe;
pub(crate) mod regexp;
pub(crate) mod utils;

// // only include if "with_wasm" feature is enabled
// #[cfg(feature = "with_wasm")]
// pub mod wasm;
// #[cfg(feature = "with_wasm")]
// pub use wasm::*;
