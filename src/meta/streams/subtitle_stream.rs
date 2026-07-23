//! Defines technical metadata for embedded subtitle streams.

use super::stream_info::StreamInfo;
use crate::meta::fields::SubtitleCodec;

/// Technical metadata for one embedded subtitle stream.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct SubtitleStream {
    /// Container-independent stream metadata.
    pub info: StreamInfo,
    /// The detected subtitle codec.
    pub codec: Option<SubtitleCodec>,
}
