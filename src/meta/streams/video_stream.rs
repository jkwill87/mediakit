//! Defines technical metadata for video streams.

use super::stream_info::StreamInfo;
use crate::meta::fields::{VideoCodec, VideoDynamicRange, VideoProfile, VideoResolution};

/// Technical metadata for one video stream.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct VideoStream {
    /// Container-independent stream metadata.
    pub info: StreamInfo,
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
