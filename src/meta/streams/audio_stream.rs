//! Defines technical metadata for audio streams.

use super::stream_info::StreamInfo;
use crate::meta::fields::{AudioCodec, AudioLayout, AudioProfile};

/// Technical metadata for one audio stream.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct AudioStream {
    /// Container-independent stream metadata.
    pub info: StreamInfo,
    /// The detected audio codec.
    pub codec: Option<AudioCodec>,
    /// The detected codec profile.
    pub profile: Option<AudioProfile>,
    /// The detected channel layout.
    pub layout: Option<AudioLayout>,
    /// The average bit rate in bits per second.
    pub bit_rate: Option<u32>,
}
