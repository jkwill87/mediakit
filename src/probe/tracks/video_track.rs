//! Defines technical metadata for video tracks.

use super::TrackInfo;
use crate::meta::fields::{VideoCodec, VideoDynamicRange, VideoProfile, VideoResolution};

/// Technical metadata for one video track.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct VideoTrack {
    /// Container-independent track metadata.
    pub info: TrackInfo,
    /// The detected video codec.
    pub codec: Option<VideoCodec>,
    /// The detected codec profile.
    pub profile: Option<VideoProfile>,
    /// The coded width in pixels.
    pub width: Option<u32>,
    /// The coded height in pixels.
    pub height: Option<u32>,
    /// The normalized resolution category.
    pub resolution: Option<VideoResolution>,
    /// The frame rate in frames per second.
    pub frame_rate: Option<f32>,
    /// The detected video dynamic range.
    pub dynamic_range: Option<VideoDynamicRange>,
}
