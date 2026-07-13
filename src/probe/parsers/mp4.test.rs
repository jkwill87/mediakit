//! Verifies bounded ISO-BMFF and QuickTime container parsing.

use super::*;

fn boxed(kind: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&u32::try_from(payload.len() + 8).unwrap().to_be_bytes());
    data.extend_from_slice(kind);
    data.extend_from_slice(payload);
    data
}

#[test]
fn box_reader_rejects_parent_overflow() {
    let mut data = 20_u32.to_be_bytes().to_vec();
    data.extend_from_slice(b"test");
    assert!(Boxes::new(&data).next().unwrap().is_err());
}

#[test]
fn parses_extended_and_open_ended_boxes() {
    let mut extended = 1_u32.to_be_bytes().to_vec();
    extended.extend_from_slice(b"wide");
    extended.extend_from_slice(&20_u64.to_be_bytes());
    extended.extend_from_slice(b"data");
    let view = Boxes::new(&extended).next().unwrap().unwrap();
    assert_eq!(view.kind, *b"wide");
    assert_eq!(view.payload, b"data");

    let mut open = 0_u32.to_be_bytes().to_vec();
    open.extend_from_slice(b"free");
    open.extend_from_slice(b"rest");
    assert_eq!(Boxes::new(&open).next().unwrap().unwrap().payload, b"rest");
}

#[test]
fn detects_quicktime_brand() {
    assert!(is_quicktime_brand(b"qt  \0\0\0\0isom"));
    assert!(!is_quicktime_brand(b"isom\0\0\0\0mp42"));
    assert_eq!(boxed(b"free", b"x").len(), 9);
}

#[test]
fn finds_nested_aac_audio_specific_config() {
    let esds = [0, 0, 0, 0, 0x03, 6, 0, 0, 0x05, 1, 2 << 3];
    assert_eq!(parse_aac_profile(&esds), Some(AudioProfile::LowComplexity));
}
