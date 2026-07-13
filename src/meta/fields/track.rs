//! Defines external and container track metadata.

use super::Language;

/// The kind of media track described by filename or container metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum TrackKind {
    /// A video track.
    Video,
    /// An audio track.
    Audio,
    /// A subtitle track.
    Subtitle,
}

/// A disposition attached to an audio or subtitle track.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TrackDisposition {
    /// Intended to be presented regardless of the viewer's language choice.
    Forced,
    /// Intended for deaf and hard-of-hearing viewers.
    Sdh,
    /// Commentary associated with the media presentation.
    Commentary,
}

impl TrackDisposition {
    /// Returns the stable filename suffix for the disposition.
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Forced => "forced",
            Self::Sdh => "sdh",
            Self::Commentary => "commentary",
        }
    }
}

impl std::fmt::Display for TrackDisposition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.suffix())
    }
}

/// Structured metadata for a media track described by a filename.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrackMetadata {
    /// The kind of track.
    pub kind: TrackKind,
    /// The track language, when declared.
    pub language: Option<Language>,
    /// A numeric discriminator retained from the source name.
    pub number: Option<u16>,
    /// Retained dispositions in source order.
    pub dispositions: Vec<TrackDisposition>,
}
