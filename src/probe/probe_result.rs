//! Defines ordered container and track metadata from probing.

use super::{AudioTrack, SubtitleTrack, Track, TrackInfo, VideoTrack};
use crate::meta::fields::MediaFormat;
use std::time::Duration;

/// Technical metadata discovered by probing a media container.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ProbeResult {
    /// The normalized media container format detected from content.
    pub container: MediaFormat,
    /// The media duration when declared by the container.
    pub duration: Option<Duration>,
    /// Supported media tracks in their native container order.
    pub tracks: Vec<Track>,
}

impl ProbeResult {
    pub(super) const fn new(container: MediaFormat) -> Self {
        Self {
            container,
            duration: None,
            tracks: Vec::new(),
        }
    }

    /// Iterates over audio tracks in container order.
    pub fn audio_tracks(&self) -> impl Iterator<Item = &AudioTrack> + Clone + '_ {
        self.tracks.iter().filter_map(|track| match track {
            Track::Audio(track) => Some(track),
            _ => None,
        })
    }

    /// Iterates over video tracks in container order.
    pub fn video_tracks(&self) -> impl Iterator<Item = &VideoTrack> + Clone + '_ {
        self.tracks.iter().filter_map(|track| match track {
            Track::Video(track) => Some(track),
            _ => None,
        })
    }

    /// Iterates over embedded subtitle tracks in container order.
    pub fn subtitle_tracks(&self) -> impl Iterator<Item = &SubtitleTrack> + Clone + '_ {
        self.tracks.iter().filter_map(|track| match track {
            Track::Subtitle(track) => Some(track),
            _ => None,
        })
    }

    /// Returns the preferred audio track.
    pub fn primary_audio_track(&self) -> Option<&AudioTrack> {
        primary_track(self.audio_tracks(), |track| &track.info)
    }

    /// Returns the preferred video track.
    pub fn primary_video_track(&self) -> Option<&VideoTrack> {
        primary_track(self.video_tracks(), |track| &track.info)
    }

    /// Returns the preferred embedded subtitle track.
    pub fn primary_subtitle_track(&self) -> Option<&SubtitleTrack> {
        primary_track(self.subtitle_tracks(), |track| &track.info)
    }
}

fn primary_track<'a, T: 'a>(
    tracks: impl Iterator<Item = &'a T> + Clone,
    info: impl Fn(&T) -> &TrackInfo,
) -> Option<&'a T> {
    tracks
        .clone()
        .find(|track| {
            let info = info(track);
            info.is_enabled && info.is_default
        })
        .or_else(|| tracks.clone().find(|track| info(track).is_enabled))
        .or_else(|| tracks.into_iter().next())
}

crate::unit_tests!("probe_result.test.rs");
