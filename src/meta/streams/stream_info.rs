//! Defines metadata shared by every media stream.

use crate::meta::fields::Language;

/// Container-independent metadata shared by every media stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct StreamInfo {
    /// Whether the container marks the stream as enabled.
    pub is_enabled: bool,
    /// Whether the container marks the stream as the default.
    pub is_default: bool,
    /// The normalized language declared by the container.
    pub language: Option<Language>,
}

impl Default for StreamInfo {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_default: false,
            language: None,
        }
    }
}
