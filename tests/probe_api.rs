//! Verifies the public container-probing contract.

use mediakit::meta::{
    fields::{Language, MediaFormat, SubtitleCodec},
    streams::{AudioStream, StreamInfo, SubtitleStream, VideoStream},
};
use mediakit::probe::{FileProber, MediaInfo, ProbeError};
use std::fs;

#[test]
fn file_prober_api_is_available_to_external_callers() {
    fn assert_result_type(_: Result<MediaInfo, ProbeError>) {}

    let path = std::env::temp_dir().join(format!("mediakit-public-probe-{}", std::process::id()));
    fs::write(&path, b"RIFF\0\0\0\0AVI ").unwrap();
    let prober = FileProber::new(&path).unwrap();
    let result = prober.probe();
    assert_eq!(result.as_ref().unwrap().container, MediaFormat::Avi);
    assert_result_type(result);
    fs::remove_file(&path).unwrap();

    let missing = std::env::temp_dir().join(format!(
        "mediakit-public-probe-missing-{}",
        std::process::id()
    ));
    let result = FileProber::new(missing);
    assert!(matches!(result, Err(ProbeError::Io(_))));
}

#[test]
fn embedded_subtitle_stream_type_is_public() {
    fn assert_audio_type(_: AudioStream) {}
    fn assert_language_type(_: Option<Language>) {}
    fn assert_info_type(_: StreamInfo) {}
    fn assert_codec_type(_: Option<SubtitleCodec>) {}
    fn assert_video_type(_: VideoStream) {}

    let stream = SubtitleStream::default();

    assert_audio_type(AudioStream::default());
    assert!(stream.info.is_enabled);
    assert!(!stream.info.is_default);
    assert_language_type(stream.info.language);
    assert_info_type(stream.info);
    assert_codec_type(stream.codec.clone());
    assert_eq!(stream.info.language, None);
    assert_eq!(stream.codec, None);
    assert_video_type(VideoStream::default());
}

#[test]
fn media_format_is_the_public_container_type() {
    fn assert_format_type(_: MediaFormat) {}

    let stream = StreamInfo::default();
    assert!(stream.is_enabled);
    assert_format_type(MediaFormat::Mkv);
}
