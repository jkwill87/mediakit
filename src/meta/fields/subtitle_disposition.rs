//! Defines standalone-subtitle disposition metadata.

/// A disposition encoded in a standalone subtitle filename.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SubtitleDisposition {
    /// Intended to be presented regardless of the viewer's language choice.
    Forced,
    /// Intended for deaf and hard-of-hearing viewers.
    Sdh,
    /// Commentary associated with the media presentation.
    Commentary,
}

impl SubtitleDisposition {
    /// Returns the stable filename suffix for the disposition.
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Forced => "forced",
            Self::Sdh => "sdh",
            Self::Commentary => "commentary",
        }
    }
}

impl std::fmt::Display for SubtitleDisposition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.suffix())
    }
}
