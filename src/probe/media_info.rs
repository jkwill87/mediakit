//! Defines ordered container and stream metadata from probing.

use crate::meta::fields::{
    AudioCodec, AudioLayout, AudioProfile, Language, VideoCodec, VideoDynamicRange, VideoProfile,
    VideoResolution,
};
use std::time::Duration;

/// Technical metadata discovered by probing a media container.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct MediaInfo {
    /// The normalized container name, such as `mkv`, `webm`, `mp4`, or `mov`.
    pub container: String,
    /// The media duration when declared by the container.
    pub duration: Option<Duration>,
    /// Audio streams in container order.
    pub audio_streams: Vec<AudioStream>,
    /// Video streams in container order.
    pub video_streams: Vec<VideoStream>,
    /// Embedded subtitle streams in container order.
    pub subtitle_streams: Vec<SubtitleStream>,
}

impl MediaInfo {
    pub(crate) fn new(container: impl Into<String>) -> Self {
        Self {
            container: container.into(),
            duration: None,
            audio_streams: Vec::new(),
            video_streams: Vec::new(),
            subtitle_streams: Vec::new(),
        }
    }

    /// Returns the preferred audio stream.
    ///
    /// Enabled default streams are preferred, followed by enabled streams and
    /// finally the first audio stream in container order.
    pub fn primary_audio_stream(&self) -> Option<&AudioStream> {
        primary_stream(&self.audio_streams, |stream| {
            (stream.is_enabled, stream.is_default)
        })
    }

    /// Returns the preferred video stream.
    ///
    /// Enabled default streams are preferred, followed by enabled streams and
    /// finally the first video stream in container order.
    pub fn primary_video_stream(&self) -> Option<&VideoStream> {
        primary_stream(&self.video_streams, |stream| {
            (stream.is_enabled, stream.is_default)
        })
    }
}

fn primary_stream<T>(streams: &[T], flags: impl Fn(&T) -> (bool, bool)) -> Option<&T> {
    streams
        .iter()
        .find(|stream| flags(stream) == (true, true))
        .or_else(|| streams.iter().find(|stream| flags(stream).0))
        .or_else(|| streams.first())
}

/// Technical metadata for one audio stream.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct AudioStream {
    /// Whether the container marks the stream as enabled.
    pub is_enabled: bool,
    /// Whether the container marks the stream as the default.
    pub is_default: bool,
    /// The normalized language declared by the container.
    pub language: Option<Language>,
    /// The detected audio codec.
    pub codec: Option<AudioCodec>,
    /// The detected codec profile.
    pub profile: Option<AudioProfile>,
    /// The detected channel layout.
    pub layout: Option<AudioLayout>,
    /// The average bit rate in bits per second.
    pub bit_rate: Option<u32>,
}

impl Default for AudioStream {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_default: false,
            language: None,
            codec: None,
            profile: None,
            layout: None,
            bit_rate: None,
        }
    }
}

/// Technical metadata for one video stream.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct VideoStream {
    /// Whether the container marks the stream as enabled.
    pub is_enabled: bool,
    /// Whether the container marks the stream as the default.
    pub is_default: bool,
    /// The normalized language declared by the container.
    pub language: Option<Language>,
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

impl Default for VideoStream {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_default: false,
            language: None,
            codec: None,
            profile: None,
            width: None,
            height: None,
            resolution: None,
            frame_rate: None,
            dynamic_range: None,
        }
    }
}

/// Technical metadata for one embedded subtitle stream.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SubtitleStream {
    /// Whether the container marks the stream as enabled.
    pub is_enabled: bool,
    /// Whether the container marks the stream as the default.
    pub is_default: bool,
    /// The normalized language declared by the container.
    pub language: Option<Language>,
    /// The container's subtitle codec identifier, such as `S_TEXT/UTF8`, `tx3g`, or `pgs`.
    pub codec: Option<String>,
}

impl Default for SubtitleStream {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_default: false,
            language: None,
            codec: None,
        }
    }
}

crate::unit_tests!("media_info.test.rs");
