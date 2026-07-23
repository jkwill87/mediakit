//! Built-in media-container detection and probe implementations.
//!
//! Format-specific signatures, framing rules, track models, and bounded iterators live here.
//! Reusable byte readers and cross-container media structures are imported from `support`.

pub(super) mod asf;
pub(super) mod avi;
pub(super) mod matroska;
pub(super) mod mp4;
pub(super) mod mpeg_ts;

use super::support::{
    audio_codec, audio_layout, avc_profile, binary, hevc_profile, pixel_dimension, subtitle_codec,
    video_codec, video_resolution, windows_media,
};
use super::{MediaInfo, ProbeInput};

/// Maximum leading bytes required by any built-in container detector.
pub(super) const DETECTION_BYTES: usize = mpeg_ts::DETECTION_BYTES;
