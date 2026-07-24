//! Defines technical metadata for audio tracks.

use super::TrackInfo;
use crate::meta::fields::{AudioCodec, AudioLayout, AudioProfile};

/// Technical metadata for one audio track.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct AudioTrack {
    /// Container-independent track metadata.
    pub info: TrackInfo,
    /// The detected audio codec.
    pub codec: Option<AudioCodec>,
    /// The detected codec profile.
    pub profile: Option<AudioProfile>,
    /// The detected channel layout.
    pub layout: Option<AudioLayout>,
    /// The average bit rate in bits per second.
    pub bit_rate: Option<u32>,
}
