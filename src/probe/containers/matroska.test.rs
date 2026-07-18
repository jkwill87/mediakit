//! Verifies bounded Matroska detection and probing.

use super::*;

#[test]
fn reads_ebml_integers_and_unknown_sizes() {
    assert_eq!(read_vint(&[0x81], false).unwrap(), Some((1, 1)));
    assert_eq!(read_vint(&[0x40, 0x7F], false).unwrap(), Some((127, 2)));
    assert_eq!(read_vint(&[0xFF], false).unwrap(), None);
    assert_eq!(
        read_vint(&[0x1A, 0x45, 0xDF, 0xA3], true).unwrap(),
        Some((EBML, 4))
    );
}

#[test]
fn rejects_invalid_and_truncated_ebml_integers() {
    assert!(read_vint(&[0], false).is_err());
    assert!(read_vint(&[0x40], false).is_err());
}

#[test]
fn maps_media_info_codec_ids() {
    assert_eq!(
        matroska_audio_codec("A_EAC3"),
        Some(AudioCodec::DolbyDigitalPlus)
    );
    assert_eq!(
        matroska_video_codec("V_MPEGH/ISO/HEVC"),
        Some(VideoCodec::H265)
    );
}

#[test]
fn retains_embedded_subtitle_codec_ids_and_flags() {
    let track = Track {
        kind: 17,
        enabled: false,
        default: true,
        codec_id: "S_TEXT/ASS".to_owned(),
        language: Some("eng".to_owned()),
        language_ietf: Some("en-US".to_owned()),
        ..Track::default()
    };
    let stream = subtitle_stream(&track);

    assert!(!stream.info.is_enabled);
    assert!(stream.info.is_default);
    assert_eq!(
        stream.info.language.map(|language| language.iso_639_1),
        Some("en")
    );
    assert_eq!(stream.codec.as_deref(), Some("S_TEXT/ASS"));
}

#[test]
fn ietf_language_overrides_legacy_language() {
    let data = [
        0x22, 0xB5, 0x9C, 0x83, b'e', b'n', b'g', 0x22, 0xB5, 0x9D, 0x85, b'e', b'n', b'-',
        b'U', b'S',
    ];
    let track = parse_track(&data).unwrap();

    assert_eq!(track.language.as_deref(), Some("eng"));
    assert_eq!(track.language_ietf.as_deref(), Some("en-US"));
    assert_eq!(
        track_language(&track).map(|language| language.iso_639_1),
        Some("en")
    );
}

#[test]
fn missing_track_flags_use_matroska_defaults() {
    let track = Track::default();
    assert!(track.enabled);
    assert!(track.default);
}
