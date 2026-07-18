//! Verifies stream selection and probed metadata behavior.

use super::*;

#[test]
fn primary_stream_prefers_default_then_enabled_then_first() {
    let mut info = MediaInfo::new(MediaFormat::Mkv);
    info.audio_streams = vec![
        AudioStream {
            info: StreamInfo {
                is_enabled: false,
                ..StreamInfo::default()
            },
            ..AudioStream::default()
        },
        AudioStream::default(),
        AudioStream {
            info: StreamInfo {
                is_default: true,
                ..StreamInfo::default()
            },
            ..AudioStream::default()
        },
    ];
    assert!(std::ptr::eq(
        info.primary_audio_stream().unwrap(),
        &info.audio_streams[2]
    ));

    info.audio_streams[2].info.is_enabled = false;
    assert!(std::ptr::eq(
        info.primary_audio_stream().unwrap(),
        &info.audio_streams[1]
    ));

    info.audio_streams[1].info.is_enabled = false;
    assert!(std::ptr::eq(
        info.primary_audio_stream().unwrap(),
        &info.audio_streams[0]
    ));
}

#[test]
fn primary_selection_applies_to_every_stream_kind() {
    let mut info = MediaInfo::new(MediaFormat::Mkv);
    info.video_streams = vec![VideoStream::default()];
    info.subtitle_streams = vec![
        SubtitleStream {
            info: StreamInfo {
                is_enabled: false,
                ..StreamInfo::default()
            },
            ..SubtitleStream::default()
        },
        SubtitleStream {
            info: StreamInfo {
                is_default: true,
                ..StreamInfo::default()
            },
            ..SubtitleStream::default()
        },
    ];

    assert!(std::ptr::eq(
        info.primary_video_stream().unwrap(),
        &info.video_streams[0]
    ));
    assert!(std::ptr::eq(
        info.primary_subtitle_stream().unwrap(),
        &info.subtitle_streams[1]
    ));
}
