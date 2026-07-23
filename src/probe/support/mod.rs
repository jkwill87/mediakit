//! Reusable support code for container probes.
//!
//! This layer owns bounded binary reads and media structures or normalization rules shared by
//! more than one container family. It does not detect or parse complete containers.

pub(super) mod binary;
mod media;
pub(super) mod windows_media;

pub(super) use media::{
    audio_codec, audio_layout, avc_profile, hevc_profile, pixel_dimension, subtitle_codec,
    video_codec, video_resolution,
};

/// Encoded width of a four-byte character code.
pub(super) const FOURCC_BYTES: usize = 4;
