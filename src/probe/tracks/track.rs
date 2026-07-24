//! Defines the ordered media-track sum type.

use super::{AudioTrack, SubtitleTrack, TrackInfo, VideoTrack};

/// A typed media track discovered in a container.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Track {
    /// A video track.
    Video(VideoTrack),
    /// An audio track.
    Audio(AudioTrack),
    /// An embedded subtitle track.
    Subtitle(SubtitleTrack),
}

impl Track {
    /// Returns the metadata shared by every track kind.
    pub const fn info(&self) -> &TrackInfo {
        match self {
            Self::Video(track) => &track.info,
            Self::Audio(track) => &track.info,
            Self::Subtitle(track) => &track.info,
        }
    }
}
