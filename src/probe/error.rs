//! Defines failures produced while probing media containers.

use std::io;
use thiserror::Error;

/// An error produced while probing a media file.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProbeError {
    /// The file does not use one of the supported container formats.
    #[error("unsupported media format")]
    UnsupportedFormat,

    /// The file identifies as a supported format but contains malformed data.
    #[error("invalid {format} data: {message}")]
    InvalidData {
        /// The detected container family.
        format: &'static str,
        /// A description of the malformed data.
        message: String,
    },

    /// The file could not be opened or read.
    #[error("failed to read media file: {0}")]
    Io(#[source] io::Error),
}

impl ProbeError {
    pub(crate) fn from_parser(format: &'static str, error: io::Error) -> Self {
        if matches!(
            error.kind(),
            io::ErrorKind::InvalidData | io::ErrorKind::UnexpectedEof
        ) {
            Self::InvalidData {
                format,
                message: error.to_string(),
            }
        } else {
            Self::Io(error)
        }
    }
}

impl From<io::Error> for ProbeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
