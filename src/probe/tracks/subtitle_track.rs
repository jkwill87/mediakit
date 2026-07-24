//! Defines technical metadata for embedded subtitle tracks.

use super::{SubtitleCodec, TrackInfo};

/// Technical metadata for one embedded subtitle track.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct SubtitleTrack {
    /// Container-independent track metadata.
    pub info: TrackInfo,
    /// The detected subtitle codec.
    pub codec: Option<SubtitleCodec>,
}
