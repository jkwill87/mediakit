//! Defines metadata shared by every media track.

use crate::meta::fields::Language;

/// Container-independent metadata shared by every media track.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct TrackInfo {
    /// Whether the container marks the track as enabled.
    pub is_enabled: bool,
    /// Whether the container marks the track as the default.
    pub is_default: bool,
    /// The normalized language declared by the container.
    pub language: Option<Language>,
}

impl Default for TrackInfo {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_default: false,
            language: None,
        }
    }
}
