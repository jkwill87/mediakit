//! Probes media containers for stream metadata tags.

use super::FileInspector;
use crate::meta::Tag;
use crate::probe::{AudioStream, FileProber, MediaInfo, VideoStream};

impl FileInspector {
    pub(super) fn inspect_media_content(mut self) -> Self {
        if !self.inspect_content {
            return self;
        }
        let Ok(prober) = FileProber::new(&self.path) else {
            return self;
        };
        let Ok(media) = prober.probe() else {
            return self;
        };

        self.tags
            .retain(|tag| !matches!(tag, Tag::Container(_) | Tag::MimeType(_)));
        let mut authoritative = probe_tags(&media);
        authoritative.append(&mut self.tags);
        self.tags = authoritative;
        self
    }
}

fn probe_tags(media: &MediaInfo) -> Vec<Tag> {
    let mut tags = vec![
        Tag::Container(media.container),
        Tag::MimeType(media.container.mime_type().to_owned()),
    ];
    if let Some(duration) = media.duration {
        tags.push(Tag::Runtime(duration.as_secs()));
    }
    if let Some(audio) = media.primary_audio_stream() {
        append_audio_tags(&mut tags, audio);
    }
    if let Some(video) = media.primary_video_stream() {
        append_video_tags(&mut tags, video);
    }
    tags
}

fn append_audio_tags(tags: &mut Vec<Tag>, stream: &AudioStream) {
    if let Some(value) = stream.bit_rate {
        tags.push(Tag::AudioBitRate(value));
    }
    if let Some(value) = &stream.codec {
        tags.push(Tag::AudioCodec(value.clone()));
    }
    if let Some(value) = &stream.profile {
        tags.push(Tag::AudioProfile(value.clone()));
    }
    if let Some(value) = &stream.layout {
        tags.push(Tag::AudioLayout(value.clone()));
    }
}

fn append_video_tags(tags: &mut Vec<Tag>, stream: &VideoStream) {
    if let Some(value) = &stream.codec {
        tags.push(Tag::VideoCodec(value.clone()));
    }
    if let Some(value) = &stream.profile {
        tags.push(Tag::VideoProfile(value.clone()));
    }
    if let Some(value) = stream.frame_rate {
        tags.push(Tag::VideoFrameRate(value));
    }
    if let Some(value) = &stream.resolution {
        tags.push(Tag::VideoResolution(value.clone()));
    }
    if let Some(value) = &stream.dynamic_range {
        tags.push(Tag::VideoDynamicRange(value.clone()));
    }
}
