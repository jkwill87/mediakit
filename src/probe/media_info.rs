//! Defines ordered container and stream metadata from probing.

use crate::meta::{
    fields::MediaFormat,
    streams::{AudioStream, StreamInfo, SubtitleStream, VideoStream},
};
use std::time::Duration;

/// Technical metadata discovered by probing a media container.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct MediaInfo {
    /// The normalized media container format detected from content.
    pub container: MediaFormat,
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
    pub(super) const fn new(container: MediaFormat) -> Self {
        Self {
            container,
            duration: None,
            audio_streams: Vec::new(),
            video_streams: Vec::new(),
            subtitle_streams: Vec::new(),
        }
    }

    /// Returns the preferred audio stream.
    ///
    /// Enabled default streams are preferred, followed by enabled streams and finally the first
    /// audio stream in container order.
    pub fn primary_audio_stream(&self) -> Option<&AudioStream> {
        primary_stream(&self.audio_streams, |stream| &stream.info)
    }

    /// Returns the preferred video stream.
    ///
    /// Enabled default streams are preferred, followed by enabled streams and finally the first
    /// video stream in container order.
    pub fn primary_video_stream(&self) -> Option<&VideoStream> {
        primary_stream(&self.video_streams, |stream| &stream.info)
    }

    /// Returns the preferred embedded subtitle stream.
    ///
    /// Enabled default streams are preferred, followed by enabled streams and finally the first
    /// subtitle stream in container order.
    pub fn primary_subtitle_stream(&self) -> Option<&SubtitleStream> {
        primary_stream(&self.subtitle_streams, |stream| &stream.info)
    }
}

fn primary_stream<T>(streams: &[T], info: impl Fn(&T) -> &StreamInfo) -> Option<&T> {
    streams
        .iter()
        .find(|stream| {
            let info = info(stream);
            info.is_enabled && info.is_default
        })
        .or_else(|| streams.iter().find(|stream| info(stream).is_enabled))
        .or_else(|| streams.first())
}

crate::unit_tests!("media_info.test.rs");
