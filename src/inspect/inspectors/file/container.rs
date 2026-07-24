//! Probes probe_result containers for technical metadata tags.

use super::FileInspector;
use crate::meta::Tag;
use crate::meta::fields::{Language, LanguageTag};
use crate::probe::{AudioTrack, FileProber, ProbeResult, VideoTrack};

impl FileInspector {
    pub(super) fn inspect_container(self) -> Self {
        if !self.inspect_content {
            return self;
        }
        let Ok(probe_result) = FileProber::new(&self.path).and_then(FileProber::probe) else {
            return self;
        };

        let mut tags = self.tags;
        tags.extend(probe_tags(&probe_result));
        Self { tags, ..self }
    }
}

fn probe_tags(probe_result: &ProbeResult) -> Vec<Tag> {
    let mut tags = vec![Tag::Container(probe_result.container)];
    if let Some(duration) = probe_result.duration {
        tags.push(Tag::Runtime(duration.as_secs()));
    }
    if let Some(audio) = probe_result.primary_audio_track() {
        tags.extend(audio_tags(audio));
    }
    if let Some(language) = summarize_languages(
        probe_result
            .audio_tracks()
            .filter_map(|track| track.info.language),
    ) {
        tags.push(Tag::AudioLanguage(language));
    }
    if let Some(video) = probe_result.primary_video_track() {
        tags.extend(video_tags(video));
    }
    tags
}

fn summarize_languages(languages: impl Iterator<Item = Language>) -> Option<LanguageTag> {
    let mut selected = None;
    for language in languages {
        match selected {
            None => selected = Some(language),
            Some(selected) if selected == language => {}
            Some(_) => return Some(LanguageTag::Multi),
        }
    }
    selected.map(LanguageTag::Language)
}

fn audio_tags(stream: &AudioTrack) -> Vec<Tag> {
    [
        stream.bit_rate.map(Tag::AudioBitRate),
        stream.codec.clone().map(Tag::AudioCodec),
        stream.profile.clone().map(Tag::AudioProfile),
        stream.layout.clone().map(Tag::AudioLayout),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn video_tags(stream: &VideoTrack) -> Vec<Tag> {
    [
        stream.codec.clone().map(Tag::VideoCodec),
        stream.profile.clone().map(Tag::VideoProfile),
        stream.frame_rate.map(Tag::VideoFrameRate),
        stream.resolution.clone().map(Tag::VideoResolution),
        stream.dynamic_range.clone().map(Tag::VideoDynamicRange),
    ]
    .into_iter()
    .flatten()
    .collect()
}
