//! Verifies stream selection and probed metadata behavior.

use super::*;

#[test]
fn primary_stream_prefers_default_then_enabled_then_first() {
    let mut info = MediaInfo::new("mkv");
    info.audio_streams = vec![
        AudioStream {
            is_enabled: false,
            ..AudioStream::default()
        },
        AudioStream::default(),
        AudioStream {
            is_default: true,
            ..AudioStream::default()
        },
    ];
    assert!(std::ptr::eq(
        info.primary_audio_stream().unwrap(),
        &info.audio_streams[2]
    ));

    info.audio_streams[2].is_enabled = false;
    assert!(std::ptr::eq(
        info.primary_audio_stream().unwrap(),
        &info.audio_streams[1]
    ));

    info.audio_streams[1].is_enabled = false;
    assert!(std::ptr::eq(
        info.primary_audio_stream().unwrap(),
        &info.audio_streams[0]
    ));
}
