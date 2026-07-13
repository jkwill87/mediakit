//! Coordinates filename and file-property inspection into metadata tags.

mod filename_metadata;
mod inspector;
mod inspectors;
mod token;
mod token_identity;
mod utils;

pub use filename_metadata::FilenameMetadata;
pub use inspector::Inspector;
pub use inspectors::file::FileInspector;
pub use inspectors::filename::FilenameInspector;
pub use token::Token;
pub use token_identity::TokenIdentity;
