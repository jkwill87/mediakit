//! Verifies bounded Matroska and WebM container parsing.

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
fn missing_track_flags_use_matroska_defaults() {
    let track = Track::default();
    assert!(track.enabled);
    assert!(track.default);
}
