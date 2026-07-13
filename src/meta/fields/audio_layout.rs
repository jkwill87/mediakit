//! Defines normalized audio channel layouts and aliases.

/// AudioLayout represents the audio channel layout of a media file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioLayout {
    /// The number of full-range audio channels, e.g. the 2 in 2.0 for stereo audio.
    pub full: u8,

    /// The number of subwoofer channels, e.g. the 1 in 5.1 for surround sound audio.
    pub sub: u8,

    /// The number of height channels, e.g. the 2 in 5.1.2 in a Dolby Atmos audio layout.
    pub height: u8,
}

impl std::fmt::Display for AudioLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.height {
            // If there are no height channels, don't include them in the string representation.
            0 => write!(f, "{}.{}", self.full, self.sub),
            // Otherwise, include the height channels in a 3-part string representation.
            _ => write!(f, "{}.{}.{}", self.full, self.sub, self.height),
        }
    }
}

crate::unit_tests!("audio_layout.test.rs");
