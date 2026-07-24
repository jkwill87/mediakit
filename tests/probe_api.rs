//! Verifies the public container-probing contract.

use mediakit::meta::fields::{Language, MediaFormat};
use mediakit::probe::{
    AudioTrack, FileProber, ProbeError, ProbeResult, SubtitleCodec, SubtitleTrack, Track,
    TrackInfo, VideoTrack,
};
use std::fs;

#[test]
fn file_prober_api_is_available_to_external_callers() {
    fn assert_result_type(_: Result<ProbeResult, ProbeError>) {}

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
fn embedded_subtitle_track_type_is_public() {
    fn assert_audio_type(_: AudioTrack) {}
    fn assert_language_type(_: Option<Language>) {}
    fn assert_info_type(_: TrackInfo) {}
    fn assert_codec_type(_: Option<SubtitleCodec>) {}
    fn assert_video_type(_: VideoTrack) {}

    let track = SubtitleTrack::default();

    assert_audio_type(AudioTrack::default());
    assert!(track.info.is_enabled);
    assert!(!track.info.is_default);
    assert_language_type(track.info.language);
    assert_info_type(track.info);
    assert_codec_type(track.codec.clone());
    assert_eq!(track.info.language, None);
    assert_eq!(track.codec, None);
    assert_video_type(VideoTrack::default());
}

#[test]
fn media_format_is_the_public_container_type() {
    fn assert_format_type(_: MediaFormat) {}

    let track = TrackInfo::default();
    assert!(track.is_enabled);
    assert_format_type(MediaFormat::Mkv);
}

#[test]
fn track_kind_is_expressed_by_the_enum_variant() {
    let track = Track::Audio(AudioTrack::default());
    assert!(matches!(track, Track::Audio(_)));
    assert!(track.info().is_enabled);
}
